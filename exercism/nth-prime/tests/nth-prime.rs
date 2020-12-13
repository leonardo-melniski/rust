use nth_prime as np;

#[test]
fn test_is_divide_by_none() {
    let value = np::is_divide_by(2, &vec![]);
    let expected = false;
    assert_eq!(value, expected);
}

#[test]
fn test_is_divide_by_true() {
    let value = np::is_divide_by(64, &vec![3, 5, 7, 8]);
    let expected = true;
    assert_eq!(value, expected);
}

#[test]
fn test_is_divide_by_false() {
    let value = np::is_divide_by(31, &vec![2, 3, 5, 7]);
    let expected = false;
    assert_eq!(value, expected);
}

#[test]
fn test_next_prime_firsts() {
    let mut value = np::next_prime(&vec![]);
    let mut expected = 2;
    assert_eq!(value, expected);

    value = np::next_prime(&vec![2]);
    expected = 3;
    assert_eq!(value, expected);

    value = np::next_prime(&vec![2, 3]);
    expected = 5;
    assert_eq!(value, expected);
}

#[test]
fn test_next_prime_11() {
    let value = np::next_prime(&vec![2, 3, 5, 7]);
    let expected = 11;
    assert_eq!(value, expected);
}

#[test]
fn test_next_prime_29() {
    let value = np::next_prime(&vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
    let expected = 29;
    assert_eq!(value, expected);
}


#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
fn test_big_prime() {
    assert_eq!(np::nth(10_000), 104_743);
}


#[test]
fn test_10th_prime() {
    assert_eq!(np::nth(10), 31);
}


#[test]
fn test_bigger_prime() {
    assert_eq!(np::nth(1_000_000), 104_743);
}
