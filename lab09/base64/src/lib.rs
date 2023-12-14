/// Functia primeste un sir de caractere pe care il encodeaza in base64
/// #Example
/// ```
///  use base64::encode;
/// let result:String=encode(b"123");
/// ````
pub fn encode(input: &[u8]) -> String {
  const ALPHABET: [char; 64] = [
      'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 
      'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 
      'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 
      'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'
  ];

  let mut result = String::new();
  let mut i = 0;

  while i < input.len() {
    let mut combined: u32=0;
    if i+2<input.len()
    {combined = (u32::from(input[i])).wrapping_shl(16) |
    (u32::from(input[i + 1])).wrapping_shl(8) |
    u32::from(input[i + 2]);
    }
    else if i+1<input.len()
    {
      combined = (u32::from(input[i])).wrapping_shl(16) |
    (u32::from(input[i + 1])).wrapping_shl(8);
    }
    else 
    {
      combined = (u32::from(input[i])).wrapping_shl(16);
    }
      let v1 = (combined >> (6*3)) & 0b111111;
      let v2 = (combined >>12 ) & 0b111111;
      let v3 = (combined >> 6) & 0b111111;
      let v4 = combined & 0b111111;
      result.push(ALPHABET[v1 as usize]);
      result.push(ALPHABET[v2 as usize]);
      if  v3==0 && i+1>=input.len()
      {
        result.push('=');
      }
      else
      {
        result.push(ALPHABET[v3 as usize]);
      }
      if  v4==0 && i+2>=input.len()
      {
        result.push('=');
      }
      else
      {
        result.push(ALPHABET[v4 as usize]);
      }

      i += 3;
  }

  result
}


#[test]
fn check_encodeN(){
    let a=b"123";
    assert_eq!(encode(a),"MTIz");
    let a=b"Teo";
    assert_eq!(encode(a),"VGVv")
    
}
#[test]
fn checkP1()
{
  let a=b"12";
  assert_eq!(encode(a),"MTI=");
  let a=b"R0";
  assert_eq!(encode(a),"UjA=");
  let a=b"R0mts";
  assert_eq!(encode(a),"UjBtdHM=");
}
#[test]
fn checkP2()
{
  let a=b"a";
  assert_eq!(encode(a),"YQ==");
  let a=b"Romania";
  assert_eq!(encode(a),"Um9tYW5pYQ==");
  
}
