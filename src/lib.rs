//! A tiny, secure, URL-friendly, unique string ID generator

#[macro_export]
macro_rules! gen {
    ($mod:tt, $len:tt, $alphabet:tt) => {
        mod $mod {
            pub const ALPHABET: &'static [u8; $len] = $alphabet;
        }

        pub fn $mod(mut size: usize) -> String {
            #[cfg(feature = "step")]
            assert!(
                $len <= u8::max_value() as usize,
                "The alphabet cannot be longer than a `u8`"
            );

            #[cfg(feature = "step")]
            let mask = ($len as usize).next_power_of_two() - 1;
            #[cfg(not(feature = "step"))]
            let mask = $len - 1;

            #[cfg(feature = "step")]
            debug_assert!($len <= mask + 1);

            #[cfg(feature = "step")]
            let step: usize = 8 * size / 5;
            #[cfg(not(feature = "step"))]
            let step = size;

            let mut bytes = vec![0u8; step];

            getrandom::getrandom(&mut bytes)
                .unwrap_or_else(|err| panic!("could not retreive random bytes: {}", err));

            let mut id = String::with_capacity(size);

            while size > 0 {
                size -= 1;
                id.push($mod::ALPHABET[bytes[size] as usize & mask].into());
            }

            id
        }
    };
}

// Bitcoin
// Nanoid `A-Za-z0-9_-` - `0-I_lO` = `123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz`
#[cfg(feature = "base58")]
gen!(
    base58,
    58,
    b"ModueSymbhaswnPr123456789ABCDEFGHNRVfgctiUvzKqYTJkLxpZXjQW"
);

// Nanoid `A-Za-z0-9_-` - `-_` = `A-Za-z0-9`
#[cfg(feature = "base62")]
gen!(
    base62,
    62,
    b"ModuleSymbhasOwnPr0123456789ABCDEFGHNRVfgctiUvzKqYTJkLxpZXIjQW"
);

// Nanoid `A-Za-z0-9_-`,
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
        let id = base58(21);

        assert_eq!(id.len(), 21);
    }

    #[test]
    #[cfg(feature = "base62")]
    fn generates_base62() {
        let id = base62(21);
        assert_eq!(id.len(), 21);
    }

    #[test]
    #[cfg(feature = "base64")]
    fn generates_base64() {
        let id = base64(21);

        assert_eq!(id.len(), 21);
    }

    #[test]
    fn generates_uid() {
        gen!(
            uid,
            64,
            b"_-0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );

        let id = uid(21);
        assert_eq!(id.len(), 21);
    }
}
