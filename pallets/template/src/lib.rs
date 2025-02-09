#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::vec::Vec;

    #[pallet::config]
    pub trait Config: frame_system::Config + TypeInfo {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[derive(Debug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo)]
    pub enum FileType {
        Pdf,
        Docx,
        Xls,
        Txt,
        Csv,
        Pptx,
        Jpg,
        Png,
        Unknown,
    }

    impl Default for FileType {
        fn default() -> Self {
            FileType::Unknown
        }
    }

    #[derive(Debug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo)]
    pub struct File {
        pub id: u64,
        pub name: Vec<u8>,
        pub file_type: FileType,
        pub size: u64,
        pub description: Vec<u8>,
    }

    #[pallet::storage]
    #[pallet::getter(fn files)]
    pub type Files<T: Config> =
        StorageMap<_, Blake2_128Concat, u64, File, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_file_id)]
    pub type NextFileId<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        FileAdded(u64),
        FileUpdated(u64),
        FileDeleted(u64),
    }

    #[pallet::error]
    pub enum Error<T> {
        FileNotFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::add_file())]
        pub fn add_file(
            origin: OriginFor<T>,
            name: Vec<u8>,
            size: u64,
            description: Vec<u8>,
            file_type: FileType,
        ) -> DispatchResult {
            let _sender = ensure_signed(origin)?;
            if NextFileId::<T>::get() == 0 {
                NextFileId::<T>::put(1);
            }
            let file_id = NextFileId::<T>::get();
            let file = File {
                id: file_id,
                name,
                file_type,
                size,
                description,
            };
            Files::<T>::insert(file_id, file);
            NextFileId::<T>::put(file_id + 1);
            Self::deposit_event(Event::FileAdded(file_id));
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::update_file())]
        pub fn update_file(
            origin: OriginFor<T>,
            file_id: u64,
            new_name: Vec<u8>,
            new_size: u64,
            new_description: Vec<u8>,
            new_file_type: FileType,
        ) -> DispatchResult {
            let _sender = ensure_signed(origin)?;
            let mut file = Files::<T>::get(file_id).ok_or(Error::<T>::FileNotFound)?;
            file.name = new_name;
            file.size = new_size;
            file.description = new_description;
            file.file_type = new_file_type;
            Files::<T>::insert(file_id, file);
            Self::deposit_event(Event::FileUpdated(file_id));
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::delete_file())]
        pub fn delete_file(origin: OriginFor<T>, file_id: u64) -> DispatchResult {
            let _sender = ensure_signed(origin)?;
            let _file = Files::<T>::get(file_id).ok_or(Error::<T>::FileNotFound)?;
            Files::<T>::remove(file_id);
            Self::deposit_event(Event::FileDeleted(file_id));
            Ok(())
        }
    }
}
