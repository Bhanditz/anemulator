use tokio::prelude::*;
use futures::prelude::*;
use tokio::prelude::Future;


macro_rules! instructions {
  (enum $enum_raw:ident; enum $enum_processed:ident; fn $fn_unextract:ident; fn $fn_raw:ident; fn $fn_processed:ident; cell $cell:ty, $cell_var:ident; @extract $(let $anvar:ident = $anexpr:expr);+ ; @match $extract:expr; @unextract $(let $unanvar:ident = $unanexpr:expr);+ ; @head $head_var:ident; @unextractout $unextract_combine:expr; @instructions $($ins:ident = $encoding:expr, $($id:ident|$size:ty|$processed_type:ty),* $proc:block);+; ) => (
      #[derive(Eq, PartialEq, Debug)]
      enum $enum_raw {$(
        $ins( $( $size ),* )
      ),+}
      #[derive(Eq, PartialEq, Debug)]
      enum $enum_processed {$(
        $ins( $( $processed_type ),* )
      ),+}
      fn $fn_raw($cell_var: $cell) -> $enum_raw {
        $(let $anvar = $anexpr);+;
        let part = $extract;
        match part {
          $( $encoding => $enum_raw::$ins($( $id as $size ),*) ),+,
          _ => panic!("Instruction problem")
        }
      }
      fn $fn_processed($cell_var: $enum_raw) -> impl Future<Item = $enum_processed, Error = F::Error> where F: Future<Item = $enum_processed>, {
        match $cell_var {
          $( $enum_raw::$ins($( $id ),*) => $proc ),+,
        }
      }
      fn $fn_unextract($cell_var: $enum_raw) -> $cell {
        $(let $unanvar = $unanexpr);+;
        match $cell_var {
          $( $enum_raw::$ins($( $id ),*) => {
            let $head_var = $encoding;
            $unextract_combine
          } ),+,
        }
      }
    )
}

type a13 = usize;
type d13 = usize;
type v13 = u16;
type Cell = u64;

const I0: u64 = 0b0000000000000_0000000000000_0000000000000_0000000000000_111111111111;
const I1: u64 = 0b0000000000000_0000000000000_0000000000000_1111111111111_000000000000;
const I2: u64 = 0b0000000000000_0000000000000_1111111111111_0000000000000_000000000000;
const I3: u64 = 0b0000000000000_1111111111111_0000000000000_0000000000000_000000000000;
const I4: u64 = 0b1111111111111_0000000000000_0000000000000_0000000000000_000000000000;

struct ExampleState;

instructions! {
  enum InstructionsRaw;
  enum InstructionsProcessed;
  fn unextract;
  fn matcher_raw;
  fn matcher_processed;
  struct ExampleState;
  cell Cell, n;
  @extract
  let extract = (n & I0);
  let r1      = (n & I1) >> 12;
  let r2      = (n & I2) >> 25;
  let r3      = (n & I3) >> 38;
  let r4      = (n & I4) >> 51;
  @match extract;
  @unextract
  let r1 = 0;
  let r2 = 0;
  let r3 = 0;
  let r4 = 0;
  let head = 0;
  @head head;
  @unextractout
  ((head as u64) & I0) + (((r1 as u64) << 12) & I1) + (((r2 as u64) << 25) & I2) + (((r3 as u64) << 38) & I3) + (((r4 as u64) << 51) & I4);
  @instructions
  Name = 0x01, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|d13|Cell { Ok(InstructionsProcessed::Name(r1 as Cell, r2 as Cell, r3 as Cell, r4 as Cell)); };
  Mame = 0x02, r1|a13|Cell, r2|a13|Cell { Ok(InstructionsProcessed::Mame(r1 as Cell, r2 as Cell)); };
  Fame = 0x03, r1|v13|v13, r2|a13|Cell { Ok(InstructionsProcessed::Fame(r1 as Cell, r2 as Cell)); };
}

#[test]
fn test_instruction_parsing() {
  let rm: u64 = 0b0000000000011_0000000000000_0000000000000_0000000000000_000000000001;
  let rn: u64 = 0b0000000000000_0000000000000_0000000000000_1000000000000_000000000010;
  let pm = InstructionsRaw::Name(0, 0, 0, 3);
  let pn = InstructionsRaw::Mame(0b1000000000000, 0);

  assert_eq!(matcher_raw(rm), pm);
  assert_eq!(matcher_raw(rn), pn);
  assert_eq!(unextract(pm), rm);
  assert_eq!(unextract(pn), rn);
  //assert_eq!(matcher_processed(InstructionsRaw::Name(0, 0, 0, 3)), InstructionsProcessed::Name(0, 0, 0, 3));
}
