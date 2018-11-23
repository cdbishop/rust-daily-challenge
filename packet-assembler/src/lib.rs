

use std::io;

fn get_message_id(msg: &String) -> Result<u32, String> {
    let bytes = msg.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
      if item == b'\t' {
          //return Ok(msg[0..i].trim().parse());
          let result: u32 = match msg[0..i].trim().parse() {
            Ok(num) => Ok(num),
            Err(_) => panic!("Failed to parse packet id"),
          };

          return Ok(result);
      }
    }

    return Err(String::from("Failed to find id in package"));
}

#[cfg(test)]
mod tests {

  #[test]
  fn message_id() {
    let test_input = "6220	1	10	Because he's the hero Gotham deserves, ";
    let id = get_message_id(&test_input[..]).unwrap();
    assert_eq!(6220, id);
  }
}