#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    
    use super::*;
    use frame_support::pallet_prelude::{*};
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config { }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn get_state_value)]
    pub(super) type State<T> = StorageValue<_, bool, OptionQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from(1_000_000))]
        pub fn change_state(origin: OriginFor<T>) -> DispatchResult {

            // 1. Validates the origin signature
            let _who = ensure_signed(origin)?;

            // 2. Switch state
            if let Some(state) = Self::get_state_value() {
                let new_state = !state;    
                State::<T>::set(Some(new_state));
            }
            else {
                State::<T>::set(Some(true));
            }
            
            Ok(())
        }
    }
}