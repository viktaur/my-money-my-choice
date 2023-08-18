#[cfg(test)]
mod tests {
	use crate::{
		mock, mock::*, BudgetDistribution, BudgetInfo, Department, Error, Event, RegisteredCitizens,
	};
	use frame_support::{assert_noop, assert_ok};

	#[test]
	fn citizen_registration_and_voting() {
		new_test_ext().execute_with(|| {
			// Go past genesis block so events get deposited
			System::set_block_number(1);

			// Register citizen 1
			assert_ok!(Voting::register_citizen(RuntimeOrigin::root(), 1));
			System::assert_last_event(Event::CitizenRegistered { who: 1 }.into());

			// Open a budget election
			assert_ok!(Voting::open_budget(RuntimeOrigin::root()));
			System::assert_last_event(Event::BudgetOpen.into());

			// Citizen 1 should now be able to vote
			assert_ok!(Voting::vote(RuntimeOrigin::signed(1), Department::Military, 10));

			// Check event
			System::assert_last_event(
				Event::CitizenVoted { who: 1, department: Department::Military, amount: 10 }.into(),
			);

			// Attempt to re-register citizen 1
			assert_noop!(
				Voting::register_citizen(RuntimeOrigin::root(), 1),
				Error::<Test>::CitizenAlreadyRegistered
			);

			// Deregister citizen 1
			assert_ok!(Voting::deregister_citizen(RuntimeOrigin::root(), 1));

			// Check event
			System::assert_last_event(Event::CitizenDeregistered { who: 1 }.into());

			// Citizen 1 cannot vote after being deregistered
			assert_noop!(
				Voting::vote(RuntimeOrigin::signed(1), Department::Military, 10),
				Error::<Test>::CitizenNotRegistered
			);
		});
	}

	#[test]
	fn multiple_citizen_registration_and_voting() {
		new_test_ext().execute_with(|| {
			System::set_block_number(1);

			assert_ok!(Voting::register_citizen(RuntimeOrigin::root(), 1));
			System::assert_last_event(Event::CitizenRegistered { who: 1 }.into());
			assert_ok!(Voting::register_citizen(RuntimeOrigin::root(), 2));
			System::assert_last_event(Event::CitizenRegistered { who: 2 }.into());

			assert_ok!(Voting::open_budget(RuntimeOrigin::root()));
			System::assert_last_event(Event::BudgetOpen.into());

			// Register citizen 3 after is budget is open (will have no funds)
			assert_ok!(Voting::register_citizen(RuntimeOrigin::root(), 3));
			System::assert_last_event(Event::CitizenRegistered { who: 3 }.into());

			assert_ok!(Voting::vote(RuntimeOrigin::signed(1), Department::Military, 10));
			System::assert_last_event(
				Event::CitizenVoted { who: 1, department: Department::Military, amount: 10 }.into(),
			);
			assert_ok!(Voting::vote(RuntimeOrigin::signed(2), Department::Infrastructure, 20));
			System::assert_last_event(
				Event::CitizenVoted { who: 2, department: Department::Infrastructure, amount: 20 }
					.into(),
			);
			assert_noop!(
				Voting::vote(RuntimeOrigin::signed(3), Department::Education, 30),
				Error::<Test>::NotEnoughVotingCredit
			);
		})
	}

	#[test]
	fn quadratic_voting() {
		new_test_ext().execute_with(|| {
			// Go past genesis block so events get deposited
			System::set_block_number(1);

			// Register citizen 1
			assert_ok!(Voting::register_citizen(RuntimeOrigin::root(), 1));
			assert_eq!(RegisteredCitizens::<Test>::get(1), Some(0));
			assert_ok!(Voting::open_budget(RuntimeOrigin::root()));

			// After budget opening, their voting credit should be 4096
			assert_eq!(RegisteredCitizens::<Test>::get(1), Some(4096));

			// Fund the military with 20 tokens
			assert_ok!(Voting::vote(RuntimeOrigin::signed(1), Department::Military, 20));
			// Check event
			System::assert_last_event(
				Event::CitizenVoted { who: 1, department: Department::Military, amount: 20 }.into(),
			);

			// Citizen 1's voting credit should be 4096 - 20^2 = 3696
			assert_eq!(RegisteredCitizens::<Test>::get(1), Some(3696));

			// Fund Education with 30 tokens (2796 left)
			assert_ok!(Voting::vote(RuntimeOrigin::signed(1), Department::Education, 30));
			assert_eq!(Voting::get_citizen_voting_credit(&1), Some(2796));
			assert_eq!(BudgetDistribution::<Test>::get(Department::Education, 1), Some(30));

			// Fund Politics with 10 tokens (2696 left)
			assert_ok!(Voting::vote(RuntimeOrigin::signed(1), Department::Politics, 10));
			assert_eq!(Voting::get_citizen_voting_credit(&1), Some(2696));
			assert_eq!(BudgetDistribution::<Test>::get(Department::Politics, 1), Some(10));

			// Attempt to fund politics again (should not be able to fund a department twice)
			assert_noop!(
				Voting::vote(RuntimeOrigin::signed(1), Department::Politics, 10),
				Error::<Test>::AlreadyVotedDepartment
			);
			assert_eq!(Voting::get_citizen_voting_credit(&1), Some(2696));
			assert_eq!(BudgetDistribution::<Test>::get(Department::Politics, 1), Some(10));

			// Fund Education with 50 tokens (196 left)
			assert_ok!(Voting::vote(RuntimeOrigin::signed(1), Department::Healthcare, 50));
			assert_eq!(Voting::get_citizen_voting_credit(&1), Some(196));
			assert_eq!(BudgetDistribution::<Test>::get(Department::Healthcare, 1), Some(50));

			// Attempt to fund with 15 tokens (not enough, 196 left)
			assert_noop!(
				Voting::vote(RuntimeOrigin::signed(1), Department::Infrastructure, 15),
				Error::<Test>::NotEnoughVotingCredit
			);
			assert_eq!(Voting::get_citizen_voting_credit(&1), Some(196));
			assert_eq!(BudgetDistribution::<Test>::get(Department::Infrastructure, 1), None);
		})
	}

	#[test]
	fn opening_and_closing_budget() {
		new_test_ext().execute_with(|| {
			System::set_block_number(1);

			assert_ok!(Voting::register_citizen(RuntimeOrigin::root(), 1));
			assert_eq!(RegisteredCitizens::<Test>::get(1), Some(0));

			// Root successfully opens budget
			assert_ok!(Voting::open_budget(RuntimeOrigin::root()));
			System::assert_last_event(Event::BudgetOpen.into());

			// Root should fail to open the budget again
			assert_noop!(
				Voting::open_budget(RuntimeOrigin::root()),
				Error::<Test>::BudgetAlreadyOpen
			);

			// Citizen 1 balance is now 4096
			assert_eq!(RegisteredCitizens::<Test>::get(1), Some(4096));

			// Voting
			assert_ok!(Voting::vote(RuntimeOrigin::signed(1), Department::Education, 30));
			assert_ok!(Voting::vote(RuntimeOrigin::signed(1), Department::Politics, 10));
			assert_ok!(Voting::vote(RuntimeOrigin::signed(1), Department::Healthcare, 50));
			assert_eq!(Voting::get_citizen_voting_credit(&1), Some(596));

			System::set_block_number(500);
			assert_noop!(
				Voting::close_budget(RuntimeOrigin::signed(1)),
				Error::<Test>::CannotCloseBeforeDeadline
			);
			assert_noop!(
				Voting::close_budget(RuntimeOrigin::root()),
				Error::<Test>::CannotCloseBeforeDeadline
			);

			System::set_block_number(1000);
			assert_noop!(
				Voting::close_budget(RuntimeOrigin::signed(1)),
				Error::<Test>::CannotCloseBeforeDeadline
			);

			// past deadline
			System::set_block_number(1001);

			assert_noop!(
				Voting::vote(RuntimeOrigin::signed(1), Department::Employment, 10),
				Error::<Test>::CannotVotePastDeadline
			);

			assert_ok!(Voting::close_budget(RuntimeOrigin::signed(1)));
			System::assert_last_event(Event::BudgetClosed.into());

			// Attempt to close again
			assert_noop!(
				Voting::close_budget(RuntimeOrigin::signed(1)),
				Error::<Test>::BudgetIsClosed
			);

			assert_eq!(Voting::balance_of(Department::Education), 30);
			assert_eq!(Voting::balance_of(Department::Politics), 10);
			assert_eq!(Voting::balance_of(Department::Healthcare), 50);
		})
	}
}
