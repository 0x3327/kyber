


/*************************************************
* Name:        verify
*
* Description: Compare two arrays for equality in constant time.
*
* Arguments:   const unsigned char *a: pointer to first byte array
*              const unsigned char *b: pointer to second byte array
*              size_t len:             length of the byte arrays
*
* Returns 0 if the byte arrays are equal, 1 otherwise
**************************************************/
pub fn verify(a: &[u8], b: &[u8], len: usize) -> bool
{
  let mut r = 0u64;
  for i in 0..len {
    r |= (a[i] ^ b[i]) as u64;
  }
  r = r.wrapping_neg() >> 63;
  r == 0
}


/*************************************************
* Name:        cmov
*
* Description: Copy len bytes from x to r if b is 1;
*              don't modify x if b is 0. Requires b to be in {0,1};
*              assumes two's complement representation of negative integers.
*              Runs in constant time.
*
* Arguments:   unsigned char *r:       pointer to output byte array
*              const unsigned char *x: pointer to input byte array
*              size_t len:             Amount of bytes to be copied
*              unsigned char b:        Condition bit; has to be in {0,1}
**************************************************/
pub fn cmov(r: &mut[u8], x: &mut[u8], len: usize, mut b: u8)
{
  b = b.wrapping_neg();
  for i in 0..len {
    r[i] ^= b & (x[i] ^ r[i]);
  }
}