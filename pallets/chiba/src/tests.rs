use crate::mock::*;
use frame_support::{assert_noop, assert_ok, dispatch::DispatchError};

#[test]
fn set_curator() {
    new_test_ext().execute_with(|| {
        assert_ok!(Chiba::set_curator(Origin::root(), CURATOR));
        assert_eq!(Chiba::curator(), CURATOR);
    });
}

#[test]
fn set_curator_bad_origin() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Chiba::set_curator(Origin::signed(ALICE), CURATOR),
            DispatchError::BadOrigin,
        );
    });
}

#[test]
fn create_collection() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());

        assert_eq!(
            last_event(),
            crate::mock::Event::chiba(crate::RawEvent::CollectionCreated(0)),
        );
    });
}

#[test]
fn mint() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());

        assert_eq!(
            last_event(),
            crate::mock::Event::chiba(crate::RawEvent::TokenMinted(0, 0)),
        );
    });
}

#[test]
fn mint_not_found() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());

        assert_noop!(
            Chiba::mint(
                Origin::signed(ALICE),
                1,
                Default::default(),
                Default::default(),
            ),
            crate::Error::<Test>::CollectionNotFound,
        );
    });
}

#[test]
fn mint_not_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());

        assert_noop!(
            Chiba::mint(
                Origin::signed(BOB),
                Default::default(),
                Default::default(),
                Default::default(),
            ),
            crate::Error::<Test>::NotCollectionOwner,
        );
    });
}

#[test]
fn appreciate() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());

        assert_ok!(Chiba::appreciate(
            Origin::signed(BOB),
            Default::default(),
            Default::default(),
            Default::default(),
        ));

        assert_eq!(
            last_event(),
            crate::mock::Event::chiba(crate::RawEvent::TokenAppreciated(0, 0, 0)),
        );
    });
}

#[test]
fn appreciate_not_found() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());

        assert_noop!(
            Chiba::appreciate(
                Origin::signed(BOB),
                1,
                Default::default(),
                Default::default(),
            ),
            crate::Error::<Test>::TokenNotFound,
        );
    });
}

#[test]
fn appreciate_low_balance() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());

        assert_noop!(
            Chiba::appreciate(
                Origin::signed(BOB),
                Default::default(),
                Default::default(),
                1 << 60 + 1,
            ),
            crate::Error::<Test>::LowBalance,
        );
    });
}

#[test]
fn toggle_display() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());

        assert_ok!(Chiba::toggle_display(
            Origin::signed(ALICE),
            Default::default(),
            Default::default(),
            Default::default(),
        ));

        assert_eq!(
            last_event(),
            crate::mock::Event::chiba(crate::RawEvent::TokenDisplayToggled(0, 0, false)),
        );
    });
}

#[test]
fn toggle_display_not_found() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());

        assert_noop!(
            Chiba::toggle_display(
                Origin::signed(ALICE),
                1,
                Default::default(),
                Default::default(),
            ),
            crate::Error::<Test>::TokenNotFound,
        );
    });
}

#[test]
fn toggle_display_not_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());

        assert_noop!(
            Chiba::toggle_display(
                Origin::signed(BOB),
                Default::default(),
                Default::default(),
                Default::default(),
            ),
            crate::Error::<Test>::NotTokenOwner,
        );
    });
}

#[test]
fn transfer() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());

        assert_ok!(Chiba::transfer(
            Origin::signed(ALICE),
            Default::default(),
            Default::default(),
            BOB,
        ));

        assert_eq!(
            last_event(),
            crate::mock::Event::chiba(crate::RawEvent::TokenTransferred(0, 0, BOB)),
        );
    });
}

#[test]
fn transfer_not_found() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());

        assert_noop!(
            Chiba::transfer(
                Origin::signed(ALICE),
                1,
                Default::default(),
                BOB,
            ),
            crate::Error::<Test>::TokenNotFound,
        );
    });
}

#[test]
fn transfer_not_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());

        assert_noop!(
            Chiba::transfer(
                Origin::signed(BOB),
                Default::default(),
                Default::default(),
                BOB,
            ),
            crate::Error::<Test>::NotTokenOwner,
        );
    });
}

#[test]
fn create_offer() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());

        assert_ok!(Chiba::create_offer(
            Origin::signed(BOB),
            Default::default(),
            Default::default(),
            Default::default(),
        ));

        assert_eq!(
            last_event(),
            crate::mock::Event::chiba(crate::RawEvent::OfferCreated(0, 0, 0, BOB)),
        );
    });
}

#[test]
fn accept_offer() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());

        assert_ok!(Chiba::create_offer(
            Origin::signed(BOB),
            Default::default(),
            Default::default(),
            Default::default(),
        ));

        assert_ok!(Chiba::accept_offer(
            Origin::signed(ALICE),
            Default::default(),
            Default::default(),
            BOB,
        ));

        assert_eq!(
            last_event(),
            crate::mock::Event::chiba(crate::RawEvent::OfferAccepted(0, 0, 221, BOB)),
        );
    });
}

#[test]
fn accept_offer_not_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());

        assert_ok!(Chiba::create_offer(
            Origin::signed(BOB),
            Default::default(),
            Default::default(),
            Default::default(),
        ));

        assert_noop!(
            Chiba::accept_offer(
                Origin::signed(BOB),
                Default::default(),
                Default::default(),
                BOB,
            ),
            crate::Error::<Test>::NotTokenOwner,
        );
    });
}

#[test]
fn cancel_offer() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());

        assert_ok!(Chiba::create_offer(
            Origin::signed(BOB),
            Default::default(),
            Default::default(),
            Default::default(),
        ));

        assert_ok!(Chiba::cancel_offer(
            Origin::signed(BOB),
            Default::default(),
            Default::default(),
        ));

        assert_eq!(
            last_event(),
            crate::mock::Event::chiba(crate::RawEvent::OfferCanceled(0, 0, 221, BOB)),
        );
    });
}

#[test]
fn report() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());

        assert_ok!(Chiba::report(
            Origin::signed(BOB),
            Default::default(),
            Default::default(),
            crate::ReportReason::Illegal,
        ));

        assert_eq!(
            last_event(),
            crate::mock::Event::chiba(crate::RawEvent::ReportReceived(
                0,
                0,
                crate::ReportReason::Illegal,
            )),
        );
    });
}

#[test]
fn accept_report() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());
        assert_ok!(Chiba::set_curator(Origin::root(), CURATOR));

        assert_ok!(Chiba::report(
            Origin::signed(BOB),
            Default::default(),
            Default::default(),
            crate::ReportReason::Illegal,
        ));

        assert_ok!(Chiba::accept_report(
            Origin::signed(CURATOR),
            Default::default(),
            Default::default(),
        ));

        assert_eq!(
            last_event(),
            crate::mock::Event::chiba(crate::RawEvent::ReportAccepted(0, 0)),
        );
    });
}

#[test]
fn clear_report() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());
        assert_ok!(Chiba::set_curator(Origin::root(), CURATOR));

        assert_ok!(Chiba::report(
            Origin::signed(BOB),
            Default::default(),
            Default::default(),
            crate::ReportReason::Illegal,
        ));

        assert_ok!(Chiba::clear_report(
            Origin::signed(CURATOR),
            Default::default(),
            Default::default(),
        ));

        assert_eq!(
            last_event(),
            crate::mock::Event::chiba(crate::RawEvent::ReportCleared(0, 0)),
        );
    });
}

#[test]
fn burn() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());
        assert_ok!(mint_default_token());

        assert_ok!(Chiba::burn(
            Origin::signed(ALICE),
            Default::default(),
            Default::default(),
        ));

        assert_eq!(
            last_event(),
            crate::mock::Event::chiba(crate::RawEvent::TokenBurned(0, 0)),
        );
    });
}
