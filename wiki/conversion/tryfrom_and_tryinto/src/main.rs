use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        if v % 2 == 0 {
            Ok(EvenNumber(v))
        } else {
            Err(())
        }
    }
}

fn main() {
    // TryFrom
    assert_eq!(EvenNumber::try_from(0), Ok(EvenNumber(0)));
    assert_eq!(EvenNumber::try_from(1), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
