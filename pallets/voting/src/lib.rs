#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::sp_runtime::traits::{Hash, TrailingZeroInput};
	use frame_support::traits::fungible::{Inspect, Mutate};
	use frame_support::traits::tokens::{Fortitude, Precision};
	use frame_support::{
		dispatch::{Dispatchable, GetDispatchInfo},
		fail,
		pallet_prelude::*,
		traits::{fungible, fungibles},
	};
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::boxed::Box;
	use strum::IntoEnumIterator;
	use strum_macros::EnumIter;

	type AssetIdOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
		<T as frame_system::Config>::AccountId,
	>>::AssetId;

	type BalanceOf<T> = <<T as Config>::NativeBalance as fungible::Inspect<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	type AssetBalanceOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	type BudgetId = u32;
	/// Points or tokens a citizen can use to vote on a budget election.
	type VotingCredit = u32;
	/// Representation of how much capital is allocated to each department.
	type Funds = u32;

	#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
	pub enum RawOrigin {
		VotingSuccess,
	}

	#[pallet::origin]
	pub type Origin = RawOrigin;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The runtime origin type.
		type RuntimeOrigin: From<RawOrigin>;

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Type to access the Balances Pallet.
		type NativeBalance: fungible::Inspect<Self::AccountId>
			+ fungible::Mutate<Self::AccountId>
			+ fungible::hold::Inspect<Self::AccountId>
			+ fungible::hold::Mutate<Self::AccountId>
			+ fungible::freeze::Inspect<Self::AccountId>
			+ fungible::freeze::Mutate<Self::AccountId>;

		/// Type to access the Assets Pallet.
		type Fungibles: fungibles::Inspect<Self::AccountId, Balance = BalanceOf<Self>>
			+ fungibles::Mutate<Self::AccountId>
			+ fungibles::Create<Self::AccountId>;

		/// A sudo-able call.
		type RuntimeCall: Parameter
			+ Dispatchable<RuntimeOrigin = <Self as Config>::RuntimeOrigin>
			+ GetDispatchInfo;

		/// The amount of voting credit given to a citizen
		type GivenVotingCredit: Get<u32>;

		/// Maximum number of possible registered users.
		type MaxRegisteredCitizens: Get<u32>;

		/// Maximum number of budget elections the system can have.
		type MaxBudgetElections: Get<u32>;

		/// How many number of blocks can the budget last for before being closed automatically
		type BudgetLifetime: Get<u32>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	pub type RegisteredCitizens<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, VotingCredit>;

	/// The information regarding the current budget election.
	#[pallet::storage]
	pub type CurrentBudgetElection<T: Config> = StorageValue<_, BudgetInfo<T>>;

	#[pallet::storage]
	pub type BudgetDistribution<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Department,
		Blake2_128Concat,
		T::AccountId, // Citizen (necessary to ensure they cannot vote the same dep twice)
		Funds,
	>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[derive(PartialEq, Clone, DebugNoBound, Encode, Decode, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct BudgetInfo<T: Config> {
		pub budget_id: BudgetId,
		/// The budget will close automatically if no citizen has done it yet, once the block has
		/// been reached
		pub deadline: BlockNumberFor<T>,
		/// The current state of the budget election.
		pub is_open: bool,
	}

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A budget election has been closed
		BudgetClosed,
		/// A budget election has been opened
		BudgetOpen,
		/// A citizen has been deregistered from the system.
		CitizenDeregistered { who: T::AccountId },
		/// A new citizen has been registered to vote.
		CitizenRegistered { who: T::AccountId },
		/// A citizen has successfully funded a department.
		CitizenVoted { who: T::AccountId, department: Department, amount: Funds },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// A citizen is attempting to fund a department they have already voted for before
		AlreadyVotedDepartment,
		/// The root is trying to open the budget when it's already open
		BudgetAlreadyOpen,
		/// A citizen is trying to vote or attempting to close a budget election when this is
		/// already closed or doesn't exist.
		BudgetIsClosed,
		/// A citizen / root is attempting to close the budget before it's due
		CannotCloseBeforeDeadline,
		/// A citizen is trying to vote past deadline
		CannotVotePastDeadline,
		/// The root is trying to register a citizen that had been previously registered. They need
		/// to be deregistered first.
		CitizenAlreadyRegistered,
		/// A citizen is trying to cast their vote or is trying to be removed, but is not found
		/// in the `RegisteredCitizens` storage.
		CitizenNotRegistered,
		/// A citizen is trying to vote but doesn't have enough voting credit left for that funding.
		NotEnoughVotingCredit,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_citizen(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			// only the root should be able to register a citizen
			ensure_root(origin)?;
			ensure!(
				!RegisteredCitizens::<T>::contains_key(&who),
				Error::<T>::CitizenAlreadyRegistered
			);
			// they are given 0 credits when registered. citizens must be registered before the
			// election is open.
			RegisteredCitizens::<T>::insert(&who, 0u32);
			Self::deposit_event(Event::<T>::CitizenRegistered { who });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn deregister_citizen(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			// only the root should be able to deregister a citizen
			ensure_root(origin)?;
			ensure!(RegisteredCitizens::<T>::contains_key(&who), Error::<T>::CitizenNotRegistered);
			RegisteredCitizens::<T>::remove(&who);
			Self::deposit_event(Event::<T>::CitizenDeregistered { who });
			Ok(())
		}

		/// A vote is an allocation of funds to a department using a citizen's voting credit.
		/// Citizens need to call this extrinsic for every department they want to fund.
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn vote(origin: OriginFor<T>, department: Department, amount: Funds) -> DispatchResult {
			// Check that the budget is marked as open
			match CurrentBudgetElection::<T>::get() {
				Some(budget_info) if budget_info.is_open => (),
				_ => fail!(Error::<T>::BudgetIsClosed),
			}

			// Check that the citizen is not trying to vote after the deadline (someone will need
			// to manually close the budget
			if Self::past_deadline() {
				fail!(Error::<T>::CannotVotePastDeadline)
			}

			let citizen = ensure_signed(origin)?;

			let credit_available: VotingCredit = match RegisteredCitizens::<T>::get(&citizen) {
				Some(credit) => credit,
				_ => fail!(Error::<T>::CitizenNotRegistered),
			};
			let credit_needed = amount.checked_pow(2).unwrap_or(u32::MAX);

			// Check whether the citizen has enough credit left to vote. Subtract credit if so.
			match credit_available.checked_sub(credit_needed) {
				Some(vp_left) => RegisteredCitizens::<T>::set(&citizen, Some(vp_left)),
				None => fail!(Error::<T>::NotEnoughVotingCredit),
			}

			// Check if a department was already funded (voted for). Otherwise update storage.
			match BudgetDistribution::<T>::get(&department, &citizen) {
				Some(_) => fail!(Error::<T>::AlreadyVotedDepartment),
				None => BudgetDistribution::<T>::set(&department, &citizen, Some(amount)),
			}

			// Deposit CitizenVote event
			Self::deposit_event(Event::<T>::CitizenVoted { who: citizen, department, amount });

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn open_budget(origin: OriginFor<T>) -> DispatchResult {
			// Only the root should be able to create a budget election.
			ensure_root(origin)?;

			// Check the current budget is not already open
			match CurrentBudgetElection::<T>::get() {
				Some(budget_info) if budget_info.is_open => fail!(Error::<T>::BudgetAlreadyOpen),
				_ => (),
			}

			// Drain the accounts of all departments
			for department in Department::iter() {
				T::NativeBalance::set_balance(&Self::get_department_acc(department), 0u32.into());
			}

			let new_id = match CurrentBudgetElection::<T>::get() {
				Some(budget_info) => budget_info.budget_id + 1,
				None => 0u32,
			};

			// Update current budget election
			CurrentBudgetElection::set(Some(BudgetInfo::<T> {
				budget_id: new_id,
				deadline: Self::get_current_block_number() + T::BudgetLifetime::get().into(),
				is_open: true,
			}));

			// Set all everyone's credit to GivenVotingCredit
			for (citizen, _) in RegisteredCitizens::<T>::iter() {
				RegisteredCitizens::<T>::set::<T::AccountId>(
					citizen,
					Some(T::GivenVotingCredit::get()),
				);
			}

			Self::deposit_event(Event::<T>::BudgetOpen);

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn close_budget(origin: OriginFor<T>) -> DispatchResult {
			// Anyone can call this function.
			ensure_signed_or_root(origin)?;

			// Ensure is past the deadline
			if !Self::past_deadline() {
				fail!(Error::<T>::CannotCloseBeforeDeadline)
			}

			// Mark budget as closed
			match CurrentBudgetElection::<T>::get() {
				Some(budget_info) if budget_info.is_open => {
					let new_budget_info = BudgetInfo {
						budget_id: budget_info.budget_id,
						deadline: budget_info.deadline,
						is_open: false,
					};
					CurrentBudgetElection::<T>::set(Some(new_budget_info))
				},
				_ => fail!(Error::<T>::BudgetIsClosed), // Non existent is also considered close
			};

			// Mint funding tokens to the departments
			for (department, _, funds) in BudgetDistribution::<T>::iter() {
				let generated_account = Self::get_department_acc(department);
				Self::mint_funds(&generated_account, funds.into())?;
			}

			Self::deposit_event(Event::<T>::BudgetClosed);

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn get_current_block_number() -> BlockNumberFor<T> {
			frame_system::Pallet::<T>::block_number()
		}

		pub fn past_deadline() -> bool {
			match CurrentBudgetElection::<T>::get() {
				Some(budget_info) => Self::get_current_block_number() >= budget_info.deadline,
				_ => false,
			}
		}

		/// Returns the information regarding a proposal
		pub fn budget_info() -> Option<BudgetInfo<T>> {
			CurrentBudgetElection::<T>::get()
		}

		pub fn mint_funds(account_id: &T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
			match T::NativeBalance::mint_into(account_id, amount) {
				Ok(_) => Ok(()),
				Err(e) => Err(e),
			}
		}

		pub fn get_department_acc(department: Department) -> T::AccountId {
			let bytes = T::Hashing::hash(&department.encode());
			T::AccountId::decode(&mut TrailingZeroInput::new(&bytes.encode()))
				.expect("we assume all bytes can be turned into some account id")
		}

		pub fn get_citizen_voting_credit(citizen: &T::AccountId) -> Option<VotingCredit> {
			RegisteredCitizens::<T>::get(citizen)
		}

		pub fn balance_of(
			department: Department,
		) -> <T::NativeBalance as Inspect<T::AccountId>>::Balance {
			T::NativeBalance::balance(&Self::get_department_acc(department))
		}
	}

	/// The set of choices for a citizen vote for.
	#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Encode, Decode, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub enum Department {
		Education,
		Employment,
		Healthcare,
		Infrastructure,
		Military,
		Politics,
		PublicGrants,
		RepayingPublicDebt,
		ScienceTech,
		SocialSecurity,
	}
}
