//! A tiny, secure, URL-friendly, unique string ID generator

#[macro_export]
macro_rules! gen {
    ($mod:tt, $len:tt, $alphabet:tt) => {
        #[doc = concat!(" Nanoid with alphabet table `", stringify!($alphabet), "`")]
        mod $mod {
            pub const MASK: usize = $len - 1;
            pub const ALPHABET: &'static [u8; $len] = $alphabet;
        }

        #[doc = concat!(" Nanoid with ", stringify!($mod))]
        pub fn $mod<const N: usize>() -> String {
            let mut bytes = [0u8; N];

            getrandom::getrandom(&mut bytes)
                .unwrap_or_else(|err| panic!("could not retreive random bytes: {}", err));

            bytes
                .iter_mut()
                .for_each(|b| *b = $mod::ALPHABET[*b as usize & $mod::MASK]);

            String::from_utf8_lossy(&bytes).to_string()
        }
    };
}

#[cfg(feature = "base58")]
gen!(
    base58,
    58,
    b"ModueSymbhaswnPr123456789ABCDEFGHNRVfgctiUvzKqYTJkLxpZXjQW"
);

#[cfg(feature = "base62")]
gen!(
    base62,
    62,
    b"ModuleSymbhasOwnPr0123456789ABCDEFGHNRVfgctiUvzKqYTJkLxpZXIjQW"
);

#[cfg(feature = "base64")]
gen!(
    base64,
    64,
    b"ModuleSymbhasOwnPr-0123456789ABCDEFGHNRVfgctiUvz_KqYTJkLxpZXIjQW"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "base58")]
    fn generates_base58() {
        let id = base58::<21>();
        println!("{}", &id);
        assert_eq!(id.len(), 21);
    }

    #[test]
    #[cfg(feature = "base62")]
    fn generates_base62() {
        let id = base62::<21>();
        println!("{}", &id);
        assert_eq!(id.len(), 21);
    }

    #[test]
    #[cfg(feature = "base64")]
    fn generates_base64() {
        let id = base64::<21>();
        println!("{}", &id);
        assert_eq!(id.len(), 21);
    }

    #[test]
    fn generates_uid() {
        gen!(
            uid,
            64,
            b"_-0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );

        let id = uid::<21>();
        println!("{}", &id);
        assert_eq!(id.len(), 21);
    }
}
