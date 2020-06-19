pub fn is_palindrome(num: u32) -> bool {
    let mut temp = num;
    let mut reversed = 0;

    while temp > 0 {
        reversed *= 10;
        reversed += temp % 10;
        temp /= 10;
    }

    return reversed == num;
}
