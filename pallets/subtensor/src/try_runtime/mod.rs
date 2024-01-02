use
{
    frame_support::
    {
        Parameter,
        Twox64Concat,
        storage_alias,
        traits::
        {
            Currency,
            LockableCurrency,
            ReservableCurrency,
            OnRuntimeUpgrade
        },
        pallet_prelude::
        {
            ValueQuery
        },
        weights::
        {
            RuntimeDbWeight
        }
    },
    sp_core::
    {
        Get
    },
    sp_runtime::
    {
        traits::
        {
            Zero
        }
    },
    sp_std::
    {
        vec,
        vec::
        {
            Vec
        }
    }
};

pub trait StubConfig: 'static
{
    type DbWeight: Get<RuntimeDbWeight>;
}

pub struct Stub<T: StubConfig>(sp_std::marker::PhantomData<T>);
impl<T: StubConfig> OnRuntimeUpgrade for Stub<T>
{
    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> 
    {
        return Ok(vec![]);
    }

    #[cfg(feature = "try-runtime")]
    fn on_runtime_upgrade() -> frame_support::weights::Weight
    {
        return T::DbWeight::get().reads_writes(0, 0);
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(account_reserved_before_bytes: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError>
    {
        return Ok(());  
    }
}

mod try_runtime
{
    use
    {
        super::
        {
            *
        },
        frame_support::
        {
            assert_ok,
            parameter_types,
            BoundedVec,
            traits::
            {
                Currency,
                OnRuntimeUpgrade,
                ReservableCurrency,
                WithdrawReasons
            }
        },
        frame_system::
        {
            pallet_prelude::
            {
                BlockNumberFor
            }
        },
        sp_core::
        {
            ConstU32
        }
    };

    struct StubConfigImpl;
    impl super::StubConfig for StubConfigImpl
    {
        type DbWeight = ();
    }
}