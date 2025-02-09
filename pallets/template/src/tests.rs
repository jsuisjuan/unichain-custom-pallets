#[cfg(test)]
mod tests {
    use super::*;
    use crate as pallet_template;
    use frame_support::{assert_noop, assert_ok, traits::{OnInitialize, OnFinalize}};
    use sp_core::H256;
    use frame_system as system;
    use sp_runtime::{testing::Header, traits::{BlakeTwo256, IdentityLookup}, BuildStorage};

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub struct TestRuntime where
            Block = Block,
            NodeBlock = Block,
            UncheckedExtrinsic = UncheckedExtrinsic,
        {
            System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
            TemplateModule: pallet_template::{Pallet, Call, Storage, Event<T>},
        }
    );

    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;
    
    impl frame_system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = (); 
        type BlockLength = (); 
        type DbWeight = (); 
        type RuntimeOrigin = RuntimeOrigin;
        type RuntimeCall = RuntimeCall;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = RuntimeEvent;
        type BlockHashCount = (); 
        type Version = (); 
        type PalletInfo = PalletInfo;
        type AccountData = (); 
        type OnNewAccount = (); 
        type OnKilledAccount = (); 
        type SystemWeightInfo = (); 
        type SS58Prefix = (); 
        type OnSetCode = ();
    }

    impl pallet_template::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type WeightInfo = (); // Para testes, pode ser um tipo vazio
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        t.into()
    }

    #[test]
    fn test_add_file() {
        new_test_ext().execute_with(|| {
            assert_ok!(TemplateModule::add_file(RuntimeOrigin::signed(1), b"test.pdf".to_vec(), 1024, b"desc".to_vec(), FileType::Pdf));
            assert_eq!(TemplateModule::files(1).unwrap().name, b"test.pdf".to_vec());
        });
    }

    #[test]
    fn test_update_file() {
        new_test_ext().execute_with(|| {
            assert_ok!(TemplateModule::add_file(RuntimeOrigin::signed(1), b"old.pdf".to_vec(), 512, b"old_desc".to_vec(), FileType::Pdf));
            assert_ok!(TemplateModule::update_file(RuntimeOrigin::signed(1), 1, b"new.pdf".to_vec(), 1024, b"new_desc".to_vec(), FileType::Txt));
            let file = TemplateModule::files(1).unwrap();
            assert_eq!(file.name, b"new.pdf".to_vec());
            assert_eq!(file.file_type, FileType::Txt);
        });
    }

    #[test]
    fn test_delete_file() {
        new_test_ext().execute_with(|| {
            assert_ok!(TemplateModule::add_file(RuntimeOrigin::signed(1), b"file.pdf".to_vec(), 256, b"desc".to_vec(), FileType::Pdf));
            assert_ok!(TemplateModule::delete_file(RuntimeOrigin::signed(1), 1));
            assert!(TemplateModule::files(1).is_none());
        });
    }

    #[test]
    fn test_next_file_id_increment() {
        new_test_ext().execute_with(|| {
            assert_ok!(TemplateModule::add_file(RuntimeOrigin::signed(1), b"file1.pdf".to_vec(), 100, b"desc".to_vec(), FileType::Pdf));
            assert_ok!(TemplateModule::add_file(RuntimeOrigin::signed(1), b"file2.pdf".to_vec(), 200, b"desc".to_vec(), FileType::Pdf));
            assert_eq!(TemplateModule::next_file_id(), 3);
        });
    }

    #[test]
    fn test_update_nonexistent_file_fails() {
        new_test_ext().execute_with(|| {
            assert_noop!(
                TemplateModule::update_file(RuntimeOrigin::signed(1), 999, b"new.pdf".to_vec(), 1024, b"desc".to_vec(), FileType::Txt),
                Error::<Test>::FileNotFound
            );
        });
    }

    #[test]
    fn test_delete_nonexistent_file_fails() {
        new_test_ext().execute_with(|| {
            assert_noop!(
                TemplateModule::delete_file(RuntimeOrigin::signed(1), 999),
                Error::<Test>::FileNotFound
            );
        });
    }
}