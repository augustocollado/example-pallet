#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    
    use super::*;
    use frame_support::pallet_prelude::{*, ValueQuery};
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config { }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn get_state_value)]
    pub(super) type State<T> = StorageValue<_, bool, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from(1_000_000))]
        pub fn change_state(origin: OriginFor<T>) -> DispatchResult {

            // 1. Validates the origin signature
            let _who = ensure_signed(origin)?;

            // 2. Switch state
            let new_state = !Self::get_state_value();
            State::<T>::set(new_state);
            
            Ok(())
        }
    }
}