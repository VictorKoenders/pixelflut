// Auto generated with gen_basic.py
// See that file for more info

use std::convert::TryInto;

pub fn parse_coordinate(buffer: &[u8]) -> Option<(u16, &[u8])> {
    match (
        buffer.get(0),
        buffer.get(1),
        buffer.get(2),
        buffer.get(3),
        buffer.get(4),
    ) {
        (Some(b'0'), Some(b' '), _, _, _) => Some((0, unsafe { buffer.get_unchecked(2..) })),
        (Some(b'1'), Some(b' '), _, _, _) => Some((1, unsafe { buffer.get_unchecked(2..) })),
        (Some(b'2'), Some(b' '), _, _, _) => Some((2, unsafe { buffer.get_unchecked(2..) })),
        (Some(b'3'), Some(b' '), _, _, _) => Some((3, unsafe { buffer.get_unchecked(2..) })),
        (Some(b'4'), Some(b' '), _, _, _) => Some((4, unsafe { buffer.get_unchecked(2..) })),
        (Some(b'5'), Some(b' '), _, _, _) => Some((5, unsafe { buffer.get_unchecked(2..) })),
        (Some(b'6'), Some(b' '), _, _, _) => Some((6, unsafe { buffer.get_unchecked(2..) })),
        (Some(b'7'), Some(b' '), _, _, _) => Some((7, unsafe { buffer.get_unchecked(2..) })),
        (Some(b'8'), Some(b' '), _, _, _) => Some((8, unsafe { buffer.get_unchecked(2..) })),
        (Some(b'9'), Some(b' '), _, _, _) => Some((9, unsafe { buffer.get_unchecked(2..) })),
        (Some(b'1'), Some(b'0'), Some(b' '), _, _) => {
            Some((10, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b' '), _, _) => {
            Some((11, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b' '), _, _) => {
            Some((12, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b' '), _, _) => {
            Some((13, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b' '), _, _) => {
            Some((14, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b' '), _, _) => {
            Some((15, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b' '), _, _) => {
            Some((16, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b' '), _, _) => {
            Some((17, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'1'), Some(b'8'), Some(b' '), _, _) => {
            Some((18, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'1'), Some(b'9'), Some(b' '), _, _) => {
            Some((19, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'2'), Some(b'0'), Some(b' '), _, _) => {
            Some((20, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'2'), Some(b'1'), Some(b' '), _, _) => {
            Some((21, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'2'), Some(b'2'), Some(b' '), _, _) => {
            Some((22, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'2'), Some(b'3'), Some(b' '), _, _) => {
            Some((23, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'2'), Some(b'4'), Some(b' '), _, _) => {
            Some((24, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'2'), Some(b'5'), Some(b' '), _, _) => {
            Some((25, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'2'), Some(b'6'), Some(b' '), _, _) => {
            Some((26, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'2'), Some(b'7'), Some(b' '), _, _) => {
            Some((27, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'2'), Some(b'8'), Some(b' '), _, _) => {
            Some((28, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'2'), Some(b'9'), Some(b' '), _, _) => {
            Some((29, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'3'), Some(b'0'), Some(b' '), _, _) => {
            Some((30, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'3'), Some(b'1'), Some(b' '), _, _) => {
            Some((31, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'3'), Some(b'2'), Some(b' '), _, _) => {
            Some((32, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'3'), Some(b'3'), Some(b' '), _, _) => {
            Some((33, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'3'), Some(b'4'), Some(b' '), _, _) => {
            Some((34, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'3'), Some(b'5'), Some(b' '), _, _) => {
            Some((35, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'3'), Some(b'6'), Some(b' '), _, _) => {
            Some((36, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'3'), Some(b'7'), Some(b' '), _, _) => {
            Some((37, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'3'), Some(b'8'), Some(b' '), _, _) => {
            Some((38, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'3'), Some(b'9'), Some(b' '), _, _) => {
            Some((39, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'4'), Some(b'0'), Some(b' '), _, _) => {
            Some((40, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'4'), Some(b'1'), Some(b' '), _, _) => {
            Some((41, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'4'), Some(b'2'), Some(b' '), _, _) => {
            Some((42, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'4'), Some(b'3'), Some(b' '), _, _) => {
            Some((43, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'4'), Some(b'4'), Some(b' '), _, _) => {
            Some((44, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'4'), Some(b'5'), Some(b' '), _, _) => {
            Some((45, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'4'), Some(b'6'), Some(b' '), _, _) => {
            Some((46, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'4'), Some(b'7'), Some(b' '), _, _) => {
            Some((47, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'4'), Some(b'8'), Some(b' '), _, _) => {
            Some((48, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'4'), Some(b'9'), Some(b' '), _, _) => {
            Some((49, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'5'), Some(b'0'), Some(b' '), _, _) => {
            Some((50, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'5'), Some(b'1'), Some(b' '), _, _) => {
            Some((51, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'5'), Some(b'2'), Some(b' '), _, _) => {
            Some((52, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'5'), Some(b'3'), Some(b' '), _, _) => {
            Some((53, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'5'), Some(b'4'), Some(b' '), _, _) => {
            Some((54, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'5'), Some(b'5'), Some(b' '), _, _) => {
            Some((55, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'5'), Some(b'6'), Some(b' '), _, _) => {
            Some((56, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'5'), Some(b'7'), Some(b' '), _, _) => {
            Some((57, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'5'), Some(b'8'), Some(b' '), _, _) => {
            Some((58, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'5'), Some(b'9'), Some(b' '), _, _) => {
            Some((59, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'6'), Some(b'0'), Some(b' '), _, _) => {
            Some((60, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'6'), Some(b'1'), Some(b' '), _, _) => {
            Some((61, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'6'), Some(b'2'), Some(b' '), _, _) => {
            Some((62, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'6'), Some(b'3'), Some(b' '), _, _) => {
            Some((63, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'6'), Some(b'4'), Some(b' '), _, _) => {
            Some((64, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'6'), Some(b'5'), Some(b' '), _, _) => {
            Some((65, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'6'), Some(b'6'), Some(b' '), _, _) => {
            Some((66, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'6'), Some(b'7'), Some(b' '), _, _) => {
            Some((67, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'6'), Some(b'8'), Some(b' '), _, _) => {
            Some((68, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'6'), Some(b'9'), Some(b' '), _, _) => {
            Some((69, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'7'), Some(b'0'), Some(b' '), _, _) => {
            Some((70, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'7'), Some(b'1'), Some(b' '), _, _) => {
            Some((71, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'7'), Some(b'2'), Some(b' '), _, _) => {
            Some((72, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'7'), Some(b'3'), Some(b' '), _, _) => {
            Some((73, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'7'), Some(b'4'), Some(b' '), _, _) => {
            Some((74, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'7'), Some(b'5'), Some(b' '), _, _) => {
            Some((75, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'7'), Some(b'6'), Some(b' '), _, _) => {
            Some((76, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'7'), Some(b'7'), Some(b' '), _, _) => {
            Some((77, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'7'), Some(b'8'), Some(b' '), _, _) => {
            Some((78, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'7'), Some(b'9'), Some(b' '), _, _) => {
            Some((79, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'8'), Some(b'0'), Some(b' '), _, _) => {
            Some((80, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'8'), Some(b'1'), Some(b' '), _, _) => {
            Some((81, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'8'), Some(b'2'), Some(b' '), _, _) => {
            Some((82, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'8'), Some(b'3'), Some(b' '), _, _) => {
            Some((83, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'8'), Some(b'4'), Some(b' '), _, _) => {
            Some((84, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'8'), Some(b'5'), Some(b' '), _, _) => {
            Some((85, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'8'), Some(b'6'), Some(b' '), _, _) => {
            Some((86, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'8'), Some(b'7'), Some(b' '), _, _) => {
            Some((87, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'8'), Some(b'8'), Some(b' '), _, _) => {
            Some((88, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'8'), Some(b'9'), Some(b' '), _, _) => {
            Some((89, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'9'), Some(b'0'), Some(b' '), _, _) => {
            Some((90, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'9'), Some(b'1'), Some(b' '), _, _) => {
            Some((91, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'9'), Some(b'2'), Some(b' '), _, _) => {
            Some((92, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'9'), Some(b'3'), Some(b' '), _, _) => {
            Some((93, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'9'), Some(b'4'), Some(b' '), _, _) => {
            Some((94, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'9'), Some(b'5'), Some(b' '), _, _) => {
            Some((95, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'9'), Some(b'6'), Some(b' '), _, _) => {
            Some((96, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'9'), Some(b'7'), Some(b' '), _, _) => {
            Some((97, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'9'), Some(b'8'), Some(b' '), _, _) => {
            Some((98, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'9'), Some(b'9'), Some(b' '), _, _) => {
            Some((99, unsafe { buffer.get_unchecked(3..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'0'), Some(b' '), _) => {
            Some((100, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'1'), Some(b' '), _) => {
            Some((101, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'2'), Some(b' '), _) => {
            Some((102, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'3'), Some(b' '), _) => {
            Some((103, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'4'), Some(b' '), _) => {
            Some((104, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'5'), Some(b' '), _) => {
            Some((105, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'6'), Some(b' '), _) => {
            Some((106, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'7'), Some(b' '), _) => {
            Some((107, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'8'), Some(b' '), _) => {
            Some((108, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'9'), Some(b' '), _) => {
            Some((109, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'0'), Some(b' '), _) => {
            Some((110, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'1'), Some(b' '), _) => {
            Some((111, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'2'), Some(b' '), _) => {
            Some((112, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'3'), Some(b' '), _) => {
            Some((113, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'4'), Some(b' '), _) => {
            Some((114, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'5'), Some(b' '), _) => {
            Some((115, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'6'), Some(b' '), _) => {
            Some((116, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'7'), Some(b' '), _) => {
            Some((117, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'8'), Some(b' '), _) => {
            Some((118, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'9'), Some(b' '), _) => {
            Some((119, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'0'), Some(b' '), _) => {
            Some((120, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'1'), Some(b' '), _) => {
            Some((121, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'2'), Some(b' '), _) => {
            Some((122, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'3'), Some(b' '), _) => {
            Some((123, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'4'), Some(b' '), _) => {
            Some((124, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'5'), Some(b' '), _) => {
            Some((125, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'6'), Some(b' '), _) => {
            Some((126, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'7'), Some(b' '), _) => {
            Some((127, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'8'), Some(b' '), _) => {
            Some((128, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'9'), Some(b' '), _) => {
            Some((129, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'0'), Some(b' '), _) => {
            Some((130, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'1'), Some(b' '), _) => {
            Some((131, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'2'), Some(b' '), _) => {
            Some((132, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'3'), Some(b' '), _) => {
            Some((133, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'4'), Some(b' '), _) => {
            Some((134, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'5'), Some(b' '), _) => {
            Some((135, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'6'), Some(b' '), _) => {
            Some((136, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'7'), Some(b' '), _) => {
            Some((137, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'8'), Some(b' '), _) => {
            Some((138, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'9'), Some(b' '), _) => {
            Some((139, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'0'), Some(b' '), _) => {
            Some((140, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'1'), Some(b' '), _) => {
            Some((141, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'2'), Some(b' '), _) => {
            Some((142, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'3'), Some(b' '), _) => {
            Some((143, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'4'), Some(b' '), _) => {
            Some((144, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'5'), Some(b' '), _) => {
            Some((145, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'6'), Some(b' '), _) => {
            Some((146, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'7'), Some(b' '), _) => {
            Some((147, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'8'), Some(b' '), _) => {
            Some((148, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'9'), Some(b' '), _) => {
            Some((149, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'0'), Some(b' '), _) => {
            Some((150, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'1'), Some(b' '), _) => {
            Some((151, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'2'), Some(b' '), _) => {
            Some((152, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'3'), Some(b' '), _) => {
            Some((153, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'4'), Some(b' '), _) => {
            Some((154, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'5'), Some(b' '), _) => {
            Some((155, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'6'), Some(b' '), _) => {
            Some((156, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'7'), Some(b' '), _) => {
            Some((157, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'8'), Some(b' '), _) => {
            Some((158, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'9'), Some(b' '), _) => {
            Some((159, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'0'), Some(b' '), _) => {
            Some((160, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'1'), Some(b' '), _) => {
            Some((161, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'2'), Some(b' '), _) => {
            Some((162, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'3'), Some(b' '), _) => {
            Some((163, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'4'), Some(b' '), _) => {
            Some((164, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'5'), Some(b' '), _) => {
            Some((165, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'6'), Some(b' '), _) => {
            Some((166, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'7'), Some(b' '), _) => {
            Some((167, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'8'), Some(b' '), _) => {
            Some((168, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'9'), Some(b' '), _) => {
            Some((169, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'0'), Some(b' '), _) => {
            Some((170, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'1'), Some(b' '), _) => {
            Some((171, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'2'), Some(b' '), _) => {
            Some((172, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'3'), Some(b' '), _) => {
            Some((173, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'4'), Some(b' '), _) => {
            Some((174, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'5'), Some(b' '), _) => {
            Some((175, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'6'), Some(b' '), _) => {
            Some((176, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'7'), Some(b' '), _) => {
            Some((177, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'8'), Some(b' '), _) => {
            Some((178, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'9'), Some(b' '), _) => {
            Some((179, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'8'), Some(b'0'), Some(b' '), _) => {
            Some((180, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'8'), Some(b'1'), Some(b' '), _) => {
            Some((181, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'8'), Some(b'2'), Some(b' '), _) => {
            Some((182, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'8'), Some(b'3'), Some(b' '), _) => {
            Some((183, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'8'), Some(b'4'), Some(b' '), _) => {
            Some((184, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'8'), Some(b'5'), Some(b' '), _) => {
            Some((185, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'8'), Some(b'6'), Some(b' '), _) => {
            Some((186, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'8'), Some(b'7'), Some(b' '), _) => {
            Some((187, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'8'), Some(b'8'), Some(b' '), _) => {
            Some((188, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'8'), Some(b'9'), Some(b' '), _) => {
            Some((189, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'9'), Some(b'0'), Some(b' '), _) => {
            Some((190, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'9'), Some(b'1'), Some(b' '), _) => {
            Some((191, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'9'), Some(b'2'), Some(b' '), _) => {
            Some((192, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'9'), Some(b'3'), Some(b' '), _) => {
            Some((193, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'9'), Some(b'4'), Some(b' '), _) => {
            Some((194, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'9'), Some(b'5'), Some(b' '), _) => {
            Some((195, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'9'), Some(b'6'), Some(b' '), _) => {
            Some((196, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'9'), Some(b'7'), Some(b' '), _) => {
            Some((197, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'9'), Some(b'8'), Some(b' '), _) => {
            Some((198, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'9'), Some(b'9'), Some(b' '), _) => {
            Some((199, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'0'), Some(b'0'), Some(b' '), _) => {
            Some((200, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'0'), Some(b'1'), Some(b' '), _) => {
            Some((201, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'0'), Some(b'2'), Some(b' '), _) => {
            Some((202, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'0'), Some(b'3'), Some(b' '), _) => {
            Some((203, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'0'), Some(b'4'), Some(b' '), _) => {
            Some((204, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'0'), Some(b'5'), Some(b' '), _) => {
            Some((205, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'0'), Some(b'6'), Some(b' '), _) => {
            Some((206, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'0'), Some(b'7'), Some(b' '), _) => {
            Some((207, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'0'), Some(b'8'), Some(b' '), _) => {
            Some((208, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'0'), Some(b'9'), Some(b' '), _) => {
            Some((209, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'1'), Some(b'0'), Some(b' '), _) => {
            Some((210, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'1'), Some(b'1'), Some(b' '), _) => {
            Some((211, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'1'), Some(b'2'), Some(b' '), _) => {
            Some((212, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'1'), Some(b'3'), Some(b' '), _) => {
            Some((213, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'1'), Some(b'4'), Some(b' '), _) => {
            Some((214, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'1'), Some(b'5'), Some(b' '), _) => {
            Some((215, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'1'), Some(b'6'), Some(b' '), _) => {
            Some((216, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'1'), Some(b'7'), Some(b' '), _) => {
            Some((217, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'1'), Some(b'8'), Some(b' '), _) => {
            Some((218, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'1'), Some(b'9'), Some(b' '), _) => {
            Some((219, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'2'), Some(b'0'), Some(b' '), _) => {
            Some((220, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'2'), Some(b'1'), Some(b' '), _) => {
            Some((221, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'2'), Some(b'2'), Some(b' '), _) => {
            Some((222, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'2'), Some(b'3'), Some(b' '), _) => {
            Some((223, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'2'), Some(b'4'), Some(b' '), _) => {
            Some((224, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'2'), Some(b'5'), Some(b' '), _) => {
            Some((225, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'2'), Some(b'6'), Some(b' '), _) => {
            Some((226, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'2'), Some(b'7'), Some(b' '), _) => {
            Some((227, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'2'), Some(b'8'), Some(b' '), _) => {
            Some((228, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'2'), Some(b'9'), Some(b' '), _) => {
            Some((229, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'3'), Some(b'0'), Some(b' '), _) => {
            Some((230, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'3'), Some(b'1'), Some(b' '), _) => {
            Some((231, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'3'), Some(b'2'), Some(b' '), _) => {
            Some((232, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'3'), Some(b'3'), Some(b' '), _) => {
            Some((233, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'3'), Some(b'4'), Some(b' '), _) => {
            Some((234, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'3'), Some(b'5'), Some(b' '), _) => {
            Some((235, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'3'), Some(b'6'), Some(b' '), _) => {
            Some((236, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'3'), Some(b'7'), Some(b' '), _) => {
            Some((237, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'3'), Some(b'8'), Some(b' '), _) => {
            Some((238, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'3'), Some(b'9'), Some(b' '), _) => {
            Some((239, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'4'), Some(b'0'), Some(b' '), _) => {
            Some((240, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'4'), Some(b'1'), Some(b' '), _) => {
            Some((241, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'4'), Some(b'2'), Some(b' '), _) => {
            Some((242, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'4'), Some(b'3'), Some(b' '), _) => {
            Some((243, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'4'), Some(b'4'), Some(b' '), _) => {
            Some((244, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'4'), Some(b'5'), Some(b' '), _) => {
            Some((245, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'4'), Some(b'6'), Some(b' '), _) => {
            Some((246, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'4'), Some(b'7'), Some(b' '), _) => {
            Some((247, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'4'), Some(b'8'), Some(b' '), _) => {
            Some((248, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'4'), Some(b'9'), Some(b' '), _) => {
            Some((249, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'5'), Some(b'0'), Some(b' '), _) => {
            Some((250, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'5'), Some(b'1'), Some(b' '), _) => {
            Some((251, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'5'), Some(b'2'), Some(b' '), _) => {
            Some((252, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'5'), Some(b'3'), Some(b' '), _) => {
            Some((253, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'5'), Some(b'4'), Some(b' '), _) => {
            Some((254, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'5'), Some(b'5'), Some(b' '), _) => {
            Some((255, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'5'), Some(b'6'), Some(b' '), _) => {
            Some((256, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'5'), Some(b'7'), Some(b' '), _) => {
            Some((257, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'5'), Some(b'8'), Some(b' '), _) => {
            Some((258, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'5'), Some(b'9'), Some(b' '), _) => {
            Some((259, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'6'), Some(b'0'), Some(b' '), _) => {
            Some((260, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'6'), Some(b'1'), Some(b' '), _) => {
            Some((261, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'6'), Some(b'2'), Some(b' '), _) => {
            Some((262, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'6'), Some(b'3'), Some(b' '), _) => {
            Some((263, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'6'), Some(b'4'), Some(b' '), _) => {
            Some((264, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'6'), Some(b'5'), Some(b' '), _) => {
            Some((265, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'6'), Some(b'6'), Some(b' '), _) => {
            Some((266, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'6'), Some(b'7'), Some(b' '), _) => {
            Some((267, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'6'), Some(b'8'), Some(b' '), _) => {
            Some((268, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'6'), Some(b'9'), Some(b' '), _) => {
            Some((269, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'7'), Some(b'0'), Some(b' '), _) => {
            Some((270, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'7'), Some(b'1'), Some(b' '), _) => {
            Some((271, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'7'), Some(b'2'), Some(b' '), _) => {
            Some((272, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'7'), Some(b'3'), Some(b' '), _) => {
            Some((273, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'7'), Some(b'4'), Some(b' '), _) => {
            Some((274, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'7'), Some(b'5'), Some(b' '), _) => {
            Some((275, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'7'), Some(b'6'), Some(b' '), _) => {
            Some((276, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'7'), Some(b'7'), Some(b' '), _) => {
            Some((277, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'7'), Some(b'8'), Some(b' '), _) => {
            Some((278, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'7'), Some(b'9'), Some(b' '), _) => {
            Some((279, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'8'), Some(b'0'), Some(b' '), _) => {
            Some((280, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'8'), Some(b'1'), Some(b' '), _) => {
            Some((281, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'8'), Some(b'2'), Some(b' '), _) => {
            Some((282, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'8'), Some(b'3'), Some(b' '), _) => {
            Some((283, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'8'), Some(b'4'), Some(b' '), _) => {
            Some((284, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'8'), Some(b'5'), Some(b' '), _) => {
            Some((285, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'8'), Some(b'6'), Some(b' '), _) => {
            Some((286, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'8'), Some(b'7'), Some(b' '), _) => {
            Some((287, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'8'), Some(b'8'), Some(b' '), _) => {
            Some((288, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'8'), Some(b'9'), Some(b' '), _) => {
            Some((289, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'9'), Some(b'0'), Some(b' '), _) => {
            Some((290, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'9'), Some(b'1'), Some(b' '), _) => {
            Some((291, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'9'), Some(b'2'), Some(b' '), _) => {
            Some((292, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'9'), Some(b'3'), Some(b' '), _) => {
            Some((293, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'9'), Some(b'4'), Some(b' '), _) => {
            Some((294, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'9'), Some(b'5'), Some(b' '), _) => {
            Some((295, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'9'), Some(b'6'), Some(b' '), _) => {
            Some((296, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'9'), Some(b'7'), Some(b' '), _) => {
            Some((297, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'9'), Some(b'8'), Some(b' '), _) => {
            Some((298, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'2'), Some(b'9'), Some(b'9'), Some(b' '), _) => {
            Some((299, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'0'), Some(b'0'), Some(b' '), _) => {
            Some((300, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'0'), Some(b'1'), Some(b' '), _) => {
            Some((301, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'0'), Some(b'2'), Some(b' '), _) => {
            Some((302, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'0'), Some(b'3'), Some(b' '), _) => {
            Some((303, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'0'), Some(b'4'), Some(b' '), _) => {
            Some((304, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'0'), Some(b'5'), Some(b' '), _) => {
            Some((305, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'0'), Some(b'6'), Some(b' '), _) => {
            Some((306, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'0'), Some(b'7'), Some(b' '), _) => {
            Some((307, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'0'), Some(b'8'), Some(b' '), _) => {
            Some((308, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'0'), Some(b'9'), Some(b' '), _) => {
            Some((309, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'1'), Some(b'0'), Some(b' '), _) => {
            Some((310, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'1'), Some(b'1'), Some(b' '), _) => {
            Some((311, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'1'), Some(b'2'), Some(b' '), _) => {
            Some((312, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'1'), Some(b'3'), Some(b' '), _) => {
            Some((313, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'1'), Some(b'4'), Some(b' '), _) => {
            Some((314, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'1'), Some(b'5'), Some(b' '), _) => {
            Some((315, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'1'), Some(b'6'), Some(b' '), _) => {
            Some((316, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'1'), Some(b'7'), Some(b' '), _) => {
            Some((317, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'1'), Some(b'8'), Some(b' '), _) => {
            Some((318, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'1'), Some(b'9'), Some(b' '), _) => {
            Some((319, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'2'), Some(b'0'), Some(b' '), _) => {
            Some((320, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'2'), Some(b'1'), Some(b' '), _) => {
            Some((321, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'2'), Some(b'2'), Some(b' '), _) => {
            Some((322, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'2'), Some(b'3'), Some(b' '), _) => {
            Some((323, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'2'), Some(b'4'), Some(b' '), _) => {
            Some((324, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'2'), Some(b'5'), Some(b' '), _) => {
            Some((325, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'2'), Some(b'6'), Some(b' '), _) => {
            Some((326, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'2'), Some(b'7'), Some(b' '), _) => {
            Some((327, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'2'), Some(b'8'), Some(b' '), _) => {
            Some((328, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'2'), Some(b'9'), Some(b' '), _) => {
            Some((329, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'3'), Some(b'0'), Some(b' '), _) => {
            Some((330, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'3'), Some(b'1'), Some(b' '), _) => {
            Some((331, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'3'), Some(b'2'), Some(b' '), _) => {
            Some((332, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'3'), Some(b'3'), Some(b' '), _) => {
            Some((333, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'3'), Some(b'4'), Some(b' '), _) => {
            Some((334, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'3'), Some(b'5'), Some(b' '), _) => {
            Some((335, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'3'), Some(b'6'), Some(b' '), _) => {
            Some((336, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'3'), Some(b'7'), Some(b' '), _) => {
            Some((337, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'3'), Some(b'8'), Some(b' '), _) => {
            Some((338, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'3'), Some(b'9'), Some(b' '), _) => {
            Some((339, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'4'), Some(b'0'), Some(b' '), _) => {
            Some((340, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'4'), Some(b'1'), Some(b' '), _) => {
            Some((341, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'4'), Some(b'2'), Some(b' '), _) => {
            Some((342, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'4'), Some(b'3'), Some(b' '), _) => {
            Some((343, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'4'), Some(b'4'), Some(b' '), _) => {
            Some((344, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'4'), Some(b'5'), Some(b' '), _) => {
            Some((345, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'4'), Some(b'6'), Some(b' '), _) => {
            Some((346, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'4'), Some(b'7'), Some(b' '), _) => {
            Some((347, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'4'), Some(b'8'), Some(b' '), _) => {
            Some((348, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'4'), Some(b'9'), Some(b' '), _) => {
            Some((349, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'5'), Some(b'0'), Some(b' '), _) => {
            Some((350, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'5'), Some(b'1'), Some(b' '), _) => {
            Some((351, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'5'), Some(b'2'), Some(b' '), _) => {
            Some((352, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'5'), Some(b'3'), Some(b' '), _) => {
            Some((353, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'5'), Some(b'4'), Some(b' '), _) => {
            Some((354, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'5'), Some(b'5'), Some(b' '), _) => {
            Some((355, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'5'), Some(b'6'), Some(b' '), _) => {
            Some((356, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'5'), Some(b'7'), Some(b' '), _) => {
            Some((357, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'5'), Some(b'8'), Some(b' '), _) => {
            Some((358, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'5'), Some(b'9'), Some(b' '), _) => {
            Some((359, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'6'), Some(b'0'), Some(b' '), _) => {
            Some((360, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'6'), Some(b'1'), Some(b' '), _) => {
            Some((361, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'6'), Some(b'2'), Some(b' '), _) => {
            Some((362, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'6'), Some(b'3'), Some(b' '), _) => {
            Some((363, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'6'), Some(b'4'), Some(b' '), _) => {
            Some((364, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'6'), Some(b'5'), Some(b' '), _) => {
            Some((365, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'6'), Some(b'6'), Some(b' '), _) => {
            Some((366, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'6'), Some(b'7'), Some(b' '), _) => {
            Some((367, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'6'), Some(b'8'), Some(b' '), _) => {
            Some((368, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'6'), Some(b'9'), Some(b' '), _) => {
            Some((369, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'7'), Some(b'0'), Some(b' '), _) => {
            Some((370, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'7'), Some(b'1'), Some(b' '), _) => {
            Some((371, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'7'), Some(b'2'), Some(b' '), _) => {
            Some((372, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'7'), Some(b'3'), Some(b' '), _) => {
            Some((373, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'7'), Some(b'4'), Some(b' '), _) => {
            Some((374, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'7'), Some(b'5'), Some(b' '), _) => {
            Some((375, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'7'), Some(b'6'), Some(b' '), _) => {
            Some((376, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'7'), Some(b'7'), Some(b' '), _) => {
            Some((377, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'7'), Some(b'8'), Some(b' '), _) => {
            Some((378, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'7'), Some(b'9'), Some(b' '), _) => {
            Some((379, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'8'), Some(b'0'), Some(b' '), _) => {
            Some((380, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'8'), Some(b'1'), Some(b' '), _) => {
            Some((381, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'8'), Some(b'2'), Some(b' '), _) => {
            Some((382, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'8'), Some(b'3'), Some(b' '), _) => {
            Some((383, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'8'), Some(b'4'), Some(b' '), _) => {
            Some((384, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'8'), Some(b'5'), Some(b' '), _) => {
            Some((385, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'8'), Some(b'6'), Some(b' '), _) => {
            Some((386, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'8'), Some(b'7'), Some(b' '), _) => {
            Some((387, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'8'), Some(b'8'), Some(b' '), _) => {
            Some((388, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'8'), Some(b'9'), Some(b' '), _) => {
            Some((389, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'9'), Some(b'0'), Some(b' '), _) => {
            Some((390, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'9'), Some(b'1'), Some(b' '), _) => {
            Some((391, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'9'), Some(b'2'), Some(b' '), _) => {
            Some((392, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'9'), Some(b'3'), Some(b' '), _) => {
            Some((393, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'9'), Some(b'4'), Some(b' '), _) => {
            Some((394, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'9'), Some(b'5'), Some(b' '), _) => {
            Some((395, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'9'), Some(b'6'), Some(b' '), _) => {
            Some((396, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'9'), Some(b'7'), Some(b' '), _) => {
            Some((397, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'9'), Some(b'8'), Some(b' '), _) => {
            Some((398, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'3'), Some(b'9'), Some(b'9'), Some(b' '), _) => {
            Some((399, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'0'), Some(b'0'), Some(b' '), _) => {
            Some((400, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'0'), Some(b'1'), Some(b' '), _) => {
            Some((401, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'0'), Some(b'2'), Some(b' '), _) => {
            Some((402, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'0'), Some(b'3'), Some(b' '), _) => {
            Some((403, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'0'), Some(b'4'), Some(b' '), _) => {
            Some((404, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'0'), Some(b'5'), Some(b' '), _) => {
            Some((405, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'0'), Some(b'6'), Some(b' '), _) => {
            Some((406, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'0'), Some(b'7'), Some(b' '), _) => {
            Some((407, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'0'), Some(b'8'), Some(b' '), _) => {
            Some((408, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'0'), Some(b'9'), Some(b' '), _) => {
            Some((409, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'1'), Some(b'0'), Some(b' '), _) => {
            Some((410, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'1'), Some(b'1'), Some(b' '), _) => {
            Some((411, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'1'), Some(b'2'), Some(b' '), _) => {
            Some((412, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'1'), Some(b'3'), Some(b' '), _) => {
            Some((413, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'1'), Some(b'4'), Some(b' '), _) => {
            Some((414, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'1'), Some(b'5'), Some(b' '), _) => {
            Some((415, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'1'), Some(b'6'), Some(b' '), _) => {
            Some((416, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'1'), Some(b'7'), Some(b' '), _) => {
            Some((417, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'1'), Some(b'8'), Some(b' '), _) => {
            Some((418, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'1'), Some(b'9'), Some(b' '), _) => {
            Some((419, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'2'), Some(b'0'), Some(b' '), _) => {
            Some((420, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'2'), Some(b'1'), Some(b' '), _) => {
            Some((421, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'2'), Some(b'2'), Some(b' '), _) => {
            Some((422, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'2'), Some(b'3'), Some(b' '), _) => {
            Some((423, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'2'), Some(b'4'), Some(b' '), _) => {
            Some((424, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'2'), Some(b'5'), Some(b' '), _) => {
            Some((425, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'2'), Some(b'6'), Some(b' '), _) => {
            Some((426, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'2'), Some(b'7'), Some(b' '), _) => {
            Some((427, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'2'), Some(b'8'), Some(b' '), _) => {
            Some((428, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'2'), Some(b'9'), Some(b' '), _) => {
            Some((429, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'3'), Some(b'0'), Some(b' '), _) => {
            Some((430, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'3'), Some(b'1'), Some(b' '), _) => {
            Some((431, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'3'), Some(b'2'), Some(b' '), _) => {
            Some((432, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'3'), Some(b'3'), Some(b' '), _) => {
            Some((433, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'3'), Some(b'4'), Some(b' '), _) => {
            Some((434, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'3'), Some(b'5'), Some(b' '), _) => {
            Some((435, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'3'), Some(b'6'), Some(b' '), _) => {
            Some((436, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'3'), Some(b'7'), Some(b' '), _) => {
            Some((437, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'3'), Some(b'8'), Some(b' '), _) => {
            Some((438, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'3'), Some(b'9'), Some(b' '), _) => {
            Some((439, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'4'), Some(b'0'), Some(b' '), _) => {
            Some((440, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'4'), Some(b'1'), Some(b' '), _) => {
            Some((441, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'4'), Some(b'2'), Some(b' '), _) => {
            Some((442, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'4'), Some(b'3'), Some(b' '), _) => {
            Some((443, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'4'), Some(b'4'), Some(b' '), _) => {
            Some((444, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'4'), Some(b'5'), Some(b' '), _) => {
            Some((445, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'4'), Some(b'6'), Some(b' '), _) => {
            Some((446, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'4'), Some(b'7'), Some(b' '), _) => {
            Some((447, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'4'), Some(b'8'), Some(b' '), _) => {
            Some((448, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'4'), Some(b'9'), Some(b' '), _) => {
            Some((449, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'5'), Some(b'0'), Some(b' '), _) => {
            Some((450, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'5'), Some(b'1'), Some(b' '), _) => {
            Some((451, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'5'), Some(b'2'), Some(b' '), _) => {
            Some((452, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'5'), Some(b'3'), Some(b' '), _) => {
            Some((453, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'5'), Some(b'4'), Some(b' '), _) => {
            Some((454, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'5'), Some(b'5'), Some(b' '), _) => {
            Some((455, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'5'), Some(b'6'), Some(b' '), _) => {
            Some((456, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'5'), Some(b'7'), Some(b' '), _) => {
            Some((457, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'5'), Some(b'8'), Some(b' '), _) => {
            Some((458, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'5'), Some(b'9'), Some(b' '), _) => {
            Some((459, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'6'), Some(b'0'), Some(b' '), _) => {
            Some((460, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'6'), Some(b'1'), Some(b' '), _) => {
            Some((461, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'6'), Some(b'2'), Some(b' '), _) => {
            Some((462, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'6'), Some(b'3'), Some(b' '), _) => {
            Some((463, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'6'), Some(b'4'), Some(b' '), _) => {
            Some((464, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'6'), Some(b'5'), Some(b' '), _) => {
            Some((465, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'6'), Some(b'6'), Some(b' '), _) => {
            Some((466, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'6'), Some(b'7'), Some(b' '), _) => {
            Some((467, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'6'), Some(b'8'), Some(b' '), _) => {
            Some((468, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'6'), Some(b'9'), Some(b' '), _) => {
            Some((469, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'7'), Some(b'0'), Some(b' '), _) => {
            Some((470, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'7'), Some(b'1'), Some(b' '), _) => {
            Some((471, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'7'), Some(b'2'), Some(b' '), _) => {
            Some((472, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'7'), Some(b'3'), Some(b' '), _) => {
            Some((473, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'7'), Some(b'4'), Some(b' '), _) => {
            Some((474, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'7'), Some(b'5'), Some(b' '), _) => {
            Some((475, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'7'), Some(b'6'), Some(b' '), _) => {
            Some((476, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'7'), Some(b'7'), Some(b' '), _) => {
            Some((477, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'7'), Some(b'8'), Some(b' '), _) => {
            Some((478, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'7'), Some(b'9'), Some(b' '), _) => {
            Some((479, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'8'), Some(b'0'), Some(b' '), _) => {
            Some((480, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'8'), Some(b'1'), Some(b' '), _) => {
            Some((481, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'8'), Some(b'2'), Some(b' '), _) => {
            Some((482, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'8'), Some(b'3'), Some(b' '), _) => {
            Some((483, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'8'), Some(b'4'), Some(b' '), _) => {
            Some((484, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'8'), Some(b'5'), Some(b' '), _) => {
            Some((485, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'8'), Some(b'6'), Some(b' '), _) => {
            Some((486, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'8'), Some(b'7'), Some(b' '), _) => {
            Some((487, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'8'), Some(b'8'), Some(b' '), _) => {
            Some((488, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'8'), Some(b'9'), Some(b' '), _) => {
            Some((489, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'9'), Some(b'0'), Some(b' '), _) => {
            Some((490, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'9'), Some(b'1'), Some(b' '), _) => {
            Some((491, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'9'), Some(b'2'), Some(b' '), _) => {
            Some((492, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'9'), Some(b'3'), Some(b' '), _) => {
            Some((493, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'9'), Some(b'4'), Some(b' '), _) => {
            Some((494, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'9'), Some(b'5'), Some(b' '), _) => {
            Some((495, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'9'), Some(b'6'), Some(b' '), _) => {
            Some((496, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'9'), Some(b'7'), Some(b' '), _) => {
            Some((497, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'9'), Some(b'8'), Some(b' '), _) => {
            Some((498, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'4'), Some(b'9'), Some(b'9'), Some(b' '), _) => {
            Some((499, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'0'), Some(b'0'), Some(b' '), _) => {
            Some((500, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'0'), Some(b'1'), Some(b' '), _) => {
            Some((501, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'0'), Some(b'2'), Some(b' '), _) => {
            Some((502, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'0'), Some(b'3'), Some(b' '), _) => {
            Some((503, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'0'), Some(b'4'), Some(b' '), _) => {
            Some((504, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'0'), Some(b'5'), Some(b' '), _) => {
            Some((505, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'0'), Some(b'6'), Some(b' '), _) => {
            Some((506, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'0'), Some(b'7'), Some(b' '), _) => {
            Some((507, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'0'), Some(b'8'), Some(b' '), _) => {
            Some((508, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'0'), Some(b'9'), Some(b' '), _) => {
            Some((509, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'1'), Some(b'0'), Some(b' '), _) => {
            Some((510, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'1'), Some(b'1'), Some(b' '), _) => {
            Some((511, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'1'), Some(b'2'), Some(b' '), _) => {
            Some((512, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'1'), Some(b'3'), Some(b' '), _) => {
            Some((513, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'1'), Some(b'4'), Some(b' '), _) => {
            Some((514, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'1'), Some(b'5'), Some(b' '), _) => {
            Some((515, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'1'), Some(b'6'), Some(b' '), _) => {
            Some((516, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'1'), Some(b'7'), Some(b' '), _) => {
            Some((517, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'1'), Some(b'8'), Some(b' '), _) => {
            Some((518, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'1'), Some(b'9'), Some(b' '), _) => {
            Some((519, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'2'), Some(b'0'), Some(b' '), _) => {
            Some((520, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'2'), Some(b'1'), Some(b' '), _) => {
            Some((521, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'2'), Some(b'2'), Some(b' '), _) => {
            Some((522, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'2'), Some(b'3'), Some(b' '), _) => {
            Some((523, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'2'), Some(b'4'), Some(b' '), _) => {
            Some((524, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'2'), Some(b'5'), Some(b' '), _) => {
            Some((525, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'2'), Some(b'6'), Some(b' '), _) => {
            Some((526, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'2'), Some(b'7'), Some(b' '), _) => {
            Some((527, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'2'), Some(b'8'), Some(b' '), _) => {
            Some((528, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'2'), Some(b'9'), Some(b' '), _) => {
            Some((529, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'3'), Some(b'0'), Some(b' '), _) => {
            Some((530, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'3'), Some(b'1'), Some(b' '), _) => {
            Some((531, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'3'), Some(b'2'), Some(b' '), _) => {
            Some((532, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'3'), Some(b'3'), Some(b' '), _) => {
            Some((533, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'3'), Some(b'4'), Some(b' '), _) => {
            Some((534, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'3'), Some(b'5'), Some(b' '), _) => {
            Some((535, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'3'), Some(b'6'), Some(b' '), _) => {
            Some((536, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'3'), Some(b'7'), Some(b' '), _) => {
            Some((537, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'3'), Some(b'8'), Some(b' '), _) => {
            Some((538, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'3'), Some(b'9'), Some(b' '), _) => {
            Some((539, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'4'), Some(b'0'), Some(b' '), _) => {
            Some((540, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'4'), Some(b'1'), Some(b' '), _) => {
            Some((541, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'4'), Some(b'2'), Some(b' '), _) => {
            Some((542, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'4'), Some(b'3'), Some(b' '), _) => {
            Some((543, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'4'), Some(b'4'), Some(b' '), _) => {
            Some((544, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'4'), Some(b'5'), Some(b' '), _) => {
            Some((545, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'4'), Some(b'6'), Some(b' '), _) => {
            Some((546, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'4'), Some(b'7'), Some(b' '), _) => {
            Some((547, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'4'), Some(b'8'), Some(b' '), _) => {
            Some((548, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'4'), Some(b'9'), Some(b' '), _) => {
            Some((549, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'5'), Some(b'0'), Some(b' '), _) => {
            Some((550, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'5'), Some(b'1'), Some(b' '), _) => {
            Some((551, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'5'), Some(b'2'), Some(b' '), _) => {
            Some((552, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'5'), Some(b'3'), Some(b' '), _) => {
            Some((553, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'5'), Some(b'4'), Some(b' '), _) => {
            Some((554, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'5'), Some(b'5'), Some(b' '), _) => {
            Some((555, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'5'), Some(b'6'), Some(b' '), _) => {
            Some((556, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'5'), Some(b'7'), Some(b' '), _) => {
            Some((557, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'5'), Some(b'8'), Some(b' '), _) => {
            Some((558, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'5'), Some(b'9'), Some(b' '), _) => {
            Some((559, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'6'), Some(b'0'), Some(b' '), _) => {
            Some((560, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'6'), Some(b'1'), Some(b' '), _) => {
            Some((561, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'6'), Some(b'2'), Some(b' '), _) => {
            Some((562, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'6'), Some(b'3'), Some(b' '), _) => {
            Some((563, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'6'), Some(b'4'), Some(b' '), _) => {
            Some((564, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'6'), Some(b'5'), Some(b' '), _) => {
            Some((565, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'6'), Some(b'6'), Some(b' '), _) => {
            Some((566, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'6'), Some(b'7'), Some(b' '), _) => {
            Some((567, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'6'), Some(b'8'), Some(b' '), _) => {
            Some((568, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'6'), Some(b'9'), Some(b' '), _) => {
            Some((569, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'7'), Some(b'0'), Some(b' '), _) => {
            Some((570, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'7'), Some(b'1'), Some(b' '), _) => {
            Some((571, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'7'), Some(b'2'), Some(b' '), _) => {
            Some((572, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'7'), Some(b'3'), Some(b' '), _) => {
            Some((573, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'7'), Some(b'4'), Some(b' '), _) => {
            Some((574, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'7'), Some(b'5'), Some(b' '), _) => {
            Some((575, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'7'), Some(b'6'), Some(b' '), _) => {
            Some((576, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'7'), Some(b'7'), Some(b' '), _) => {
            Some((577, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'7'), Some(b'8'), Some(b' '), _) => {
            Some((578, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'7'), Some(b'9'), Some(b' '), _) => {
            Some((579, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'8'), Some(b'0'), Some(b' '), _) => {
            Some((580, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'8'), Some(b'1'), Some(b' '), _) => {
            Some((581, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'8'), Some(b'2'), Some(b' '), _) => {
            Some((582, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'8'), Some(b'3'), Some(b' '), _) => {
            Some((583, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'8'), Some(b'4'), Some(b' '), _) => {
            Some((584, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'8'), Some(b'5'), Some(b' '), _) => {
            Some((585, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'8'), Some(b'6'), Some(b' '), _) => {
            Some((586, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'8'), Some(b'7'), Some(b' '), _) => {
            Some((587, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'8'), Some(b'8'), Some(b' '), _) => {
            Some((588, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'8'), Some(b'9'), Some(b' '), _) => {
            Some((589, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'9'), Some(b'0'), Some(b' '), _) => {
            Some((590, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'9'), Some(b'1'), Some(b' '), _) => {
            Some((591, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'9'), Some(b'2'), Some(b' '), _) => {
            Some((592, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'9'), Some(b'3'), Some(b' '), _) => {
            Some((593, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'9'), Some(b'4'), Some(b' '), _) => {
            Some((594, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'9'), Some(b'5'), Some(b' '), _) => {
            Some((595, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'9'), Some(b'6'), Some(b' '), _) => {
            Some((596, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'9'), Some(b'7'), Some(b' '), _) => {
            Some((597, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'9'), Some(b'8'), Some(b' '), _) => {
            Some((598, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'5'), Some(b'9'), Some(b'9'), Some(b' '), _) => {
            Some((599, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'0'), Some(b'0'), Some(b' '), _) => {
            Some((600, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'0'), Some(b'1'), Some(b' '), _) => {
            Some((601, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'0'), Some(b'2'), Some(b' '), _) => {
            Some((602, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'0'), Some(b'3'), Some(b' '), _) => {
            Some((603, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'0'), Some(b'4'), Some(b' '), _) => {
            Some((604, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'0'), Some(b'5'), Some(b' '), _) => {
            Some((605, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'0'), Some(b'6'), Some(b' '), _) => {
            Some((606, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'0'), Some(b'7'), Some(b' '), _) => {
            Some((607, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'0'), Some(b'8'), Some(b' '), _) => {
            Some((608, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'0'), Some(b'9'), Some(b' '), _) => {
            Some((609, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'1'), Some(b'0'), Some(b' '), _) => {
            Some((610, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'1'), Some(b'1'), Some(b' '), _) => {
            Some((611, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'1'), Some(b'2'), Some(b' '), _) => {
            Some((612, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'1'), Some(b'3'), Some(b' '), _) => {
            Some((613, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'1'), Some(b'4'), Some(b' '), _) => {
            Some((614, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'1'), Some(b'5'), Some(b' '), _) => {
            Some((615, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'1'), Some(b'6'), Some(b' '), _) => {
            Some((616, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'1'), Some(b'7'), Some(b' '), _) => {
            Some((617, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'1'), Some(b'8'), Some(b' '), _) => {
            Some((618, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'1'), Some(b'9'), Some(b' '), _) => {
            Some((619, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'2'), Some(b'0'), Some(b' '), _) => {
            Some((620, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'2'), Some(b'1'), Some(b' '), _) => {
            Some((621, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'2'), Some(b'2'), Some(b' '), _) => {
            Some((622, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'2'), Some(b'3'), Some(b' '), _) => {
            Some((623, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'2'), Some(b'4'), Some(b' '), _) => {
            Some((624, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'2'), Some(b'5'), Some(b' '), _) => {
            Some((625, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'2'), Some(b'6'), Some(b' '), _) => {
            Some((626, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'2'), Some(b'7'), Some(b' '), _) => {
            Some((627, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'2'), Some(b'8'), Some(b' '), _) => {
            Some((628, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'2'), Some(b'9'), Some(b' '), _) => {
            Some((629, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'3'), Some(b'0'), Some(b' '), _) => {
            Some((630, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'3'), Some(b'1'), Some(b' '), _) => {
            Some((631, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'3'), Some(b'2'), Some(b' '), _) => {
            Some((632, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'3'), Some(b'3'), Some(b' '), _) => {
            Some((633, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'3'), Some(b'4'), Some(b' '), _) => {
            Some((634, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'3'), Some(b'5'), Some(b' '), _) => {
            Some((635, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'3'), Some(b'6'), Some(b' '), _) => {
            Some((636, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'3'), Some(b'7'), Some(b' '), _) => {
            Some((637, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'3'), Some(b'8'), Some(b' '), _) => {
            Some((638, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'3'), Some(b'9'), Some(b' '), _) => {
            Some((639, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'4'), Some(b'0'), Some(b' '), _) => {
            Some((640, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'4'), Some(b'1'), Some(b' '), _) => {
            Some((641, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'4'), Some(b'2'), Some(b' '), _) => {
            Some((642, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'4'), Some(b'3'), Some(b' '), _) => {
            Some((643, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'4'), Some(b'4'), Some(b' '), _) => {
            Some((644, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'4'), Some(b'5'), Some(b' '), _) => {
            Some((645, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'4'), Some(b'6'), Some(b' '), _) => {
            Some((646, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'4'), Some(b'7'), Some(b' '), _) => {
            Some((647, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'4'), Some(b'8'), Some(b' '), _) => {
            Some((648, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'4'), Some(b'9'), Some(b' '), _) => {
            Some((649, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'5'), Some(b'0'), Some(b' '), _) => {
            Some((650, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'5'), Some(b'1'), Some(b' '), _) => {
            Some((651, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'5'), Some(b'2'), Some(b' '), _) => {
            Some((652, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'5'), Some(b'3'), Some(b' '), _) => {
            Some((653, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'5'), Some(b'4'), Some(b' '), _) => {
            Some((654, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'5'), Some(b'5'), Some(b' '), _) => {
            Some((655, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'5'), Some(b'6'), Some(b' '), _) => {
            Some((656, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'5'), Some(b'7'), Some(b' '), _) => {
            Some((657, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'5'), Some(b'8'), Some(b' '), _) => {
            Some((658, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'5'), Some(b'9'), Some(b' '), _) => {
            Some((659, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'6'), Some(b'0'), Some(b' '), _) => {
            Some((660, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'6'), Some(b'1'), Some(b' '), _) => {
            Some((661, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'6'), Some(b'2'), Some(b' '), _) => {
            Some((662, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'6'), Some(b'3'), Some(b' '), _) => {
            Some((663, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'6'), Some(b'4'), Some(b' '), _) => {
            Some((664, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'6'), Some(b'5'), Some(b' '), _) => {
            Some((665, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'6'), Some(b'6'), Some(b' '), _) => {
            Some((666, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'6'), Some(b'7'), Some(b' '), _) => {
            Some((667, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'6'), Some(b'8'), Some(b' '), _) => {
            Some((668, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'6'), Some(b'9'), Some(b' '), _) => {
            Some((669, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'7'), Some(b'0'), Some(b' '), _) => {
            Some((670, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'7'), Some(b'1'), Some(b' '), _) => {
            Some((671, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'7'), Some(b'2'), Some(b' '), _) => {
            Some((672, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'7'), Some(b'3'), Some(b' '), _) => {
            Some((673, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'7'), Some(b'4'), Some(b' '), _) => {
            Some((674, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'7'), Some(b'5'), Some(b' '), _) => {
            Some((675, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'7'), Some(b'6'), Some(b' '), _) => {
            Some((676, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'7'), Some(b'7'), Some(b' '), _) => {
            Some((677, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'7'), Some(b'8'), Some(b' '), _) => {
            Some((678, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'7'), Some(b'9'), Some(b' '), _) => {
            Some((679, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'8'), Some(b'0'), Some(b' '), _) => {
            Some((680, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'8'), Some(b'1'), Some(b' '), _) => {
            Some((681, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'8'), Some(b'2'), Some(b' '), _) => {
            Some((682, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'8'), Some(b'3'), Some(b' '), _) => {
            Some((683, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'8'), Some(b'4'), Some(b' '), _) => {
            Some((684, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'8'), Some(b'5'), Some(b' '), _) => {
            Some((685, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'8'), Some(b'6'), Some(b' '), _) => {
            Some((686, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'8'), Some(b'7'), Some(b' '), _) => {
            Some((687, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'8'), Some(b'8'), Some(b' '), _) => {
            Some((688, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'8'), Some(b'9'), Some(b' '), _) => {
            Some((689, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'9'), Some(b'0'), Some(b' '), _) => {
            Some((690, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'9'), Some(b'1'), Some(b' '), _) => {
            Some((691, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'9'), Some(b'2'), Some(b' '), _) => {
            Some((692, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'9'), Some(b'3'), Some(b' '), _) => {
            Some((693, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'9'), Some(b'4'), Some(b' '), _) => {
            Some((694, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'9'), Some(b'5'), Some(b' '), _) => {
            Some((695, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'9'), Some(b'6'), Some(b' '), _) => {
            Some((696, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'9'), Some(b'7'), Some(b' '), _) => {
            Some((697, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'9'), Some(b'8'), Some(b' '), _) => {
            Some((698, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'6'), Some(b'9'), Some(b'9'), Some(b' '), _) => {
            Some((699, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'0'), Some(b'0'), Some(b' '), _) => {
            Some((700, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'0'), Some(b'1'), Some(b' '), _) => {
            Some((701, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'0'), Some(b'2'), Some(b' '), _) => {
            Some((702, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'0'), Some(b'3'), Some(b' '), _) => {
            Some((703, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'0'), Some(b'4'), Some(b' '), _) => {
            Some((704, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'0'), Some(b'5'), Some(b' '), _) => {
            Some((705, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'0'), Some(b'6'), Some(b' '), _) => {
            Some((706, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'0'), Some(b'7'), Some(b' '), _) => {
            Some((707, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'0'), Some(b'8'), Some(b' '), _) => {
            Some((708, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'0'), Some(b'9'), Some(b' '), _) => {
            Some((709, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'1'), Some(b'0'), Some(b' '), _) => {
            Some((710, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'1'), Some(b'1'), Some(b' '), _) => {
            Some((711, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'1'), Some(b'2'), Some(b' '), _) => {
            Some((712, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'1'), Some(b'3'), Some(b' '), _) => {
            Some((713, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'1'), Some(b'4'), Some(b' '), _) => {
            Some((714, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'1'), Some(b'5'), Some(b' '), _) => {
            Some((715, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'1'), Some(b'6'), Some(b' '), _) => {
            Some((716, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'1'), Some(b'7'), Some(b' '), _) => {
            Some((717, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'1'), Some(b'8'), Some(b' '), _) => {
            Some((718, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'1'), Some(b'9'), Some(b' '), _) => {
            Some((719, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'2'), Some(b'0'), Some(b' '), _) => {
            Some((720, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'2'), Some(b'1'), Some(b' '), _) => {
            Some((721, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'2'), Some(b'2'), Some(b' '), _) => {
            Some((722, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'2'), Some(b'3'), Some(b' '), _) => {
            Some((723, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'2'), Some(b'4'), Some(b' '), _) => {
            Some((724, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'2'), Some(b'5'), Some(b' '), _) => {
            Some((725, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'2'), Some(b'6'), Some(b' '), _) => {
            Some((726, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'2'), Some(b'7'), Some(b' '), _) => {
            Some((727, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'2'), Some(b'8'), Some(b' '), _) => {
            Some((728, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'2'), Some(b'9'), Some(b' '), _) => {
            Some((729, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'3'), Some(b'0'), Some(b' '), _) => {
            Some((730, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'3'), Some(b'1'), Some(b' '), _) => {
            Some((731, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'3'), Some(b'2'), Some(b' '), _) => {
            Some((732, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'3'), Some(b'3'), Some(b' '), _) => {
            Some((733, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'3'), Some(b'4'), Some(b' '), _) => {
            Some((734, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'3'), Some(b'5'), Some(b' '), _) => {
            Some((735, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'3'), Some(b'6'), Some(b' '), _) => {
            Some((736, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'3'), Some(b'7'), Some(b' '), _) => {
            Some((737, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'3'), Some(b'8'), Some(b' '), _) => {
            Some((738, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'3'), Some(b'9'), Some(b' '), _) => {
            Some((739, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'4'), Some(b'0'), Some(b' '), _) => {
            Some((740, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'4'), Some(b'1'), Some(b' '), _) => {
            Some((741, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'4'), Some(b'2'), Some(b' '), _) => {
            Some((742, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'4'), Some(b'3'), Some(b' '), _) => {
            Some((743, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'4'), Some(b'4'), Some(b' '), _) => {
            Some((744, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'4'), Some(b'5'), Some(b' '), _) => {
            Some((745, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'4'), Some(b'6'), Some(b' '), _) => {
            Some((746, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'4'), Some(b'7'), Some(b' '), _) => {
            Some((747, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'4'), Some(b'8'), Some(b' '), _) => {
            Some((748, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'4'), Some(b'9'), Some(b' '), _) => {
            Some((749, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'5'), Some(b'0'), Some(b' '), _) => {
            Some((750, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'5'), Some(b'1'), Some(b' '), _) => {
            Some((751, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'5'), Some(b'2'), Some(b' '), _) => {
            Some((752, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'5'), Some(b'3'), Some(b' '), _) => {
            Some((753, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'5'), Some(b'4'), Some(b' '), _) => {
            Some((754, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'5'), Some(b'5'), Some(b' '), _) => {
            Some((755, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'5'), Some(b'6'), Some(b' '), _) => {
            Some((756, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'5'), Some(b'7'), Some(b' '), _) => {
            Some((757, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'5'), Some(b'8'), Some(b' '), _) => {
            Some((758, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'5'), Some(b'9'), Some(b' '), _) => {
            Some((759, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'6'), Some(b'0'), Some(b' '), _) => {
            Some((760, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'6'), Some(b'1'), Some(b' '), _) => {
            Some((761, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'6'), Some(b'2'), Some(b' '), _) => {
            Some((762, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'6'), Some(b'3'), Some(b' '), _) => {
            Some((763, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'6'), Some(b'4'), Some(b' '), _) => {
            Some((764, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'6'), Some(b'5'), Some(b' '), _) => {
            Some((765, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'6'), Some(b'6'), Some(b' '), _) => {
            Some((766, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'6'), Some(b'7'), Some(b' '), _) => {
            Some((767, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'6'), Some(b'8'), Some(b' '), _) => {
            Some((768, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'6'), Some(b'9'), Some(b' '), _) => {
            Some((769, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'7'), Some(b'0'), Some(b' '), _) => {
            Some((770, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'7'), Some(b'1'), Some(b' '), _) => {
            Some((771, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'7'), Some(b'2'), Some(b' '), _) => {
            Some((772, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'7'), Some(b'3'), Some(b' '), _) => {
            Some((773, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'7'), Some(b'4'), Some(b' '), _) => {
            Some((774, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'7'), Some(b'5'), Some(b' '), _) => {
            Some((775, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'7'), Some(b'6'), Some(b' '), _) => {
            Some((776, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'7'), Some(b'7'), Some(b' '), _) => {
            Some((777, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'7'), Some(b'8'), Some(b' '), _) => {
            Some((778, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'7'), Some(b'9'), Some(b' '), _) => {
            Some((779, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'8'), Some(b'0'), Some(b' '), _) => {
            Some((780, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'8'), Some(b'1'), Some(b' '), _) => {
            Some((781, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'8'), Some(b'2'), Some(b' '), _) => {
            Some((782, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'8'), Some(b'3'), Some(b' '), _) => {
            Some((783, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'8'), Some(b'4'), Some(b' '), _) => {
            Some((784, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'8'), Some(b'5'), Some(b' '), _) => {
            Some((785, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'8'), Some(b'6'), Some(b' '), _) => {
            Some((786, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'8'), Some(b'7'), Some(b' '), _) => {
            Some((787, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'8'), Some(b'8'), Some(b' '), _) => {
            Some((788, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'8'), Some(b'9'), Some(b' '), _) => {
            Some((789, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'9'), Some(b'0'), Some(b' '), _) => {
            Some((790, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'9'), Some(b'1'), Some(b' '), _) => {
            Some((791, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'9'), Some(b'2'), Some(b' '), _) => {
            Some((792, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'9'), Some(b'3'), Some(b' '), _) => {
            Some((793, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'9'), Some(b'4'), Some(b' '), _) => {
            Some((794, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'9'), Some(b'5'), Some(b' '), _) => {
            Some((795, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'9'), Some(b'6'), Some(b' '), _) => {
            Some((796, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'9'), Some(b'7'), Some(b' '), _) => {
            Some((797, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'9'), Some(b'8'), Some(b' '), _) => {
            Some((798, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'7'), Some(b'9'), Some(b'9'), Some(b' '), _) => {
            Some((799, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'0'), Some(b'0'), Some(b' '), _) => {
            Some((800, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'0'), Some(b'1'), Some(b' '), _) => {
            Some((801, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'0'), Some(b'2'), Some(b' '), _) => {
            Some((802, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'0'), Some(b'3'), Some(b' '), _) => {
            Some((803, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'0'), Some(b'4'), Some(b' '), _) => {
            Some((804, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'0'), Some(b'5'), Some(b' '), _) => {
            Some((805, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'0'), Some(b'6'), Some(b' '), _) => {
            Some((806, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'0'), Some(b'7'), Some(b' '), _) => {
            Some((807, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'0'), Some(b'8'), Some(b' '), _) => {
            Some((808, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'0'), Some(b'9'), Some(b' '), _) => {
            Some((809, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'1'), Some(b'0'), Some(b' '), _) => {
            Some((810, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'1'), Some(b'1'), Some(b' '), _) => {
            Some((811, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'1'), Some(b'2'), Some(b' '), _) => {
            Some((812, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'1'), Some(b'3'), Some(b' '), _) => {
            Some((813, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'1'), Some(b'4'), Some(b' '), _) => {
            Some((814, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'1'), Some(b'5'), Some(b' '), _) => {
            Some((815, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'1'), Some(b'6'), Some(b' '), _) => {
            Some((816, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'1'), Some(b'7'), Some(b' '), _) => {
            Some((817, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'1'), Some(b'8'), Some(b' '), _) => {
            Some((818, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'1'), Some(b'9'), Some(b' '), _) => {
            Some((819, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'2'), Some(b'0'), Some(b' '), _) => {
            Some((820, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'2'), Some(b'1'), Some(b' '), _) => {
            Some((821, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'2'), Some(b'2'), Some(b' '), _) => {
            Some((822, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'2'), Some(b'3'), Some(b' '), _) => {
            Some((823, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'2'), Some(b'4'), Some(b' '), _) => {
            Some((824, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'2'), Some(b'5'), Some(b' '), _) => {
            Some((825, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'2'), Some(b'6'), Some(b' '), _) => {
            Some((826, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'2'), Some(b'7'), Some(b' '), _) => {
            Some((827, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'2'), Some(b'8'), Some(b' '), _) => {
            Some((828, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'2'), Some(b'9'), Some(b' '), _) => {
            Some((829, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'3'), Some(b'0'), Some(b' '), _) => {
            Some((830, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'3'), Some(b'1'), Some(b' '), _) => {
            Some((831, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'3'), Some(b'2'), Some(b' '), _) => {
            Some((832, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'3'), Some(b'3'), Some(b' '), _) => {
            Some((833, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'3'), Some(b'4'), Some(b' '), _) => {
            Some((834, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'3'), Some(b'5'), Some(b' '), _) => {
            Some((835, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'3'), Some(b'6'), Some(b' '), _) => {
            Some((836, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'3'), Some(b'7'), Some(b' '), _) => {
            Some((837, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'3'), Some(b'8'), Some(b' '), _) => {
            Some((838, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'3'), Some(b'9'), Some(b' '), _) => {
            Some((839, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'4'), Some(b'0'), Some(b' '), _) => {
            Some((840, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'4'), Some(b'1'), Some(b' '), _) => {
            Some((841, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'4'), Some(b'2'), Some(b' '), _) => {
            Some((842, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'4'), Some(b'3'), Some(b' '), _) => {
            Some((843, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'4'), Some(b'4'), Some(b' '), _) => {
            Some((844, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'4'), Some(b'5'), Some(b' '), _) => {
            Some((845, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'4'), Some(b'6'), Some(b' '), _) => {
            Some((846, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'4'), Some(b'7'), Some(b' '), _) => {
            Some((847, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'4'), Some(b'8'), Some(b' '), _) => {
            Some((848, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'4'), Some(b'9'), Some(b' '), _) => {
            Some((849, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'5'), Some(b'0'), Some(b' '), _) => {
            Some((850, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'5'), Some(b'1'), Some(b' '), _) => {
            Some((851, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'5'), Some(b'2'), Some(b' '), _) => {
            Some((852, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'5'), Some(b'3'), Some(b' '), _) => {
            Some((853, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'5'), Some(b'4'), Some(b' '), _) => {
            Some((854, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'5'), Some(b'5'), Some(b' '), _) => {
            Some((855, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'5'), Some(b'6'), Some(b' '), _) => {
            Some((856, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'5'), Some(b'7'), Some(b' '), _) => {
            Some((857, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'5'), Some(b'8'), Some(b' '), _) => {
            Some((858, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'5'), Some(b'9'), Some(b' '), _) => {
            Some((859, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'6'), Some(b'0'), Some(b' '), _) => {
            Some((860, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'6'), Some(b'1'), Some(b' '), _) => {
            Some((861, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'6'), Some(b'2'), Some(b' '), _) => {
            Some((862, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'6'), Some(b'3'), Some(b' '), _) => {
            Some((863, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'6'), Some(b'4'), Some(b' '), _) => {
            Some((864, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'6'), Some(b'5'), Some(b' '), _) => {
            Some((865, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'6'), Some(b'6'), Some(b' '), _) => {
            Some((866, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'6'), Some(b'7'), Some(b' '), _) => {
            Some((867, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'6'), Some(b'8'), Some(b' '), _) => {
            Some((868, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'6'), Some(b'9'), Some(b' '), _) => {
            Some((869, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'7'), Some(b'0'), Some(b' '), _) => {
            Some((870, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'7'), Some(b'1'), Some(b' '), _) => {
            Some((871, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'7'), Some(b'2'), Some(b' '), _) => {
            Some((872, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'7'), Some(b'3'), Some(b' '), _) => {
            Some((873, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'7'), Some(b'4'), Some(b' '), _) => {
            Some((874, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'7'), Some(b'5'), Some(b' '), _) => {
            Some((875, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'7'), Some(b'6'), Some(b' '), _) => {
            Some((876, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'7'), Some(b'7'), Some(b' '), _) => {
            Some((877, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'7'), Some(b'8'), Some(b' '), _) => {
            Some((878, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'7'), Some(b'9'), Some(b' '), _) => {
            Some((879, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'8'), Some(b'0'), Some(b' '), _) => {
            Some((880, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'8'), Some(b'1'), Some(b' '), _) => {
            Some((881, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'8'), Some(b'2'), Some(b' '), _) => {
            Some((882, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'8'), Some(b'3'), Some(b' '), _) => {
            Some((883, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'8'), Some(b'4'), Some(b' '), _) => {
            Some((884, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'8'), Some(b'5'), Some(b' '), _) => {
            Some((885, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'8'), Some(b'6'), Some(b' '), _) => {
            Some((886, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'8'), Some(b'7'), Some(b' '), _) => {
            Some((887, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'8'), Some(b'8'), Some(b' '), _) => {
            Some((888, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'8'), Some(b'9'), Some(b' '), _) => {
            Some((889, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'9'), Some(b'0'), Some(b' '), _) => {
            Some((890, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'9'), Some(b'1'), Some(b' '), _) => {
            Some((891, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'9'), Some(b'2'), Some(b' '), _) => {
            Some((892, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'9'), Some(b'3'), Some(b' '), _) => {
            Some((893, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'9'), Some(b'4'), Some(b' '), _) => {
            Some((894, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'9'), Some(b'5'), Some(b' '), _) => {
            Some((895, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'9'), Some(b'6'), Some(b' '), _) => {
            Some((896, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'9'), Some(b'7'), Some(b' '), _) => {
            Some((897, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'9'), Some(b'8'), Some(b' '), _) => {
            Some((898, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'8'), Some(b'9'), Some(b'9'), Some(b' '), _) => {
            Some((899, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'0'), Some(b'0'), Some(b' '), _) => {
            Some((900, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'0'), Some(b'1'), Some(b' '), _) => {
            Some((901, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'0'), Some(b'2'), Some(b' '), _) => {
            Some((902, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'0'), Some(b'3'), Some(b' '), _) => {
            Some((903, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'0'), Some(b'4'), Some(b' '), _) => {
            Some((904, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'0'), Some(b'5'), Some(b' '), _) => {
            Some((905, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'0'), Some(b'6'), Some(b' '), _) => {
            Some((906, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'0'), Some(b'7'), Some(b' '), _) => {
            Some((907, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'0'), Some(b'8'), Some(b' '), _) => {
            Some((908, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'0'), Some(b'9'), Some(b' '), _) => {
            Some((909, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'1'), Some(b'0'), Some(b' '), _) => {
            Some((910, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'1'), Some(b'1'), Some(b' '), _) => {
            Some((911, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'1'), Some(b'2'), Some(b' '), _) => {
            Some((912, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'1'), Some(b'3'), Some(b' '), _) => {
            Some((913, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'1'), Some(b'4'), Some(b' '), _) => {
            Some((914, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'1'), Some(b'5'), Some(b' '), _) => {
            Some((915, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'1'), Some(b'6'), Some(b' '), _) => {
            Some((916, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'1'), Some(b'7'), Some(b' '), _) => {
            Some((917, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'1'), Some(b'8'), Some(b' '), _) => {
            Some((918, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'1'), Some(b'9'), Some(b' '), _) => {
            Some((919, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'2'), Some(b'0'), Some(b' '), _) => {
            Some((920, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'2'), Some(b'1'), Some(b' '), _) => {
            Some((921, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'2'), Some(b'2'), Some(b' '), _) => {
            Some((922, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'2'), Some(b'3'), Some(b' '), _) => {
            Some((923, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'2'), Some(b'4'), Some(b' '), _) => {
            Some((924, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'2'), Some(b'5'), Some(b' '), _) => {
            Some((925, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'2'), Some(b'6'), Some(b' '), _) => {
            Some((926, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'2'), Some(b'7'), Some(b' '), _) => {
            Some((927, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'2'), Some(b'8'), Some(b' '), _) => {
            Some((928, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'2'), Some(b'9'), Some(b' '), _) => {
            Some((929, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'3'), Some(b'0'), Some(b' '), _) => {
            Some((930, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'3'), Some(b'1'), Some(b' '), _) => {
            Some((931, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'3'), Some(b'2'), Some(b' '), _) => {
            Some((932, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'3'), Some(b'3'), Some(b' '), _) => {
            Some((933, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'3'), Some(b'4'), Some(b' '), _) => {
            Some((934, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'3'), Some(b'5'), Some(b' '), _) => {
            Some((935, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'3'), Some(b'6'), Some(b' '), _) => {
            Some((936, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'3'), Some(b'7'), Some(b' '), _) => {
            Some((937, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'3'), Some(b'8'), Some(b' '), _) => {
            Some((938, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'3'), Some(b'9'), Some(b' '), _) => {
            Some((939, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'4'), Some(b'0'), Some(b' '), _) => {
            Some((940, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'4'), Some(b'1'), Some(b' '), _) => {
            Some((941, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'4'), Some(b'2'), Some(b' '), _) => {
            Some((942, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'4'), Some(b'3'), Some(b' '), _) => {
            Some((943, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'4'), Some(b'4'), Some(b' '), _) => {
            Some((944, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'4'), Some(b'5'), Some(b' '), _) => {
            Some((945, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'4'), Some(b'6'), Some(b' '), _) => {
            Some((946, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'4'), Some(b'7'), Some(b' '), _) => {
            Some((947, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'4'), Some(b'8'), Some(b' '), _) => {
            Some((948, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'4'), Some(b'9'), Some(b' '), _) => {
            Some((949, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'5'), Some(b'0'), Some(b' '), _) => {
            Some((950, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'5'), Some(b'1'), Some(b' '), _) => {
            Some((951, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'5'), Some(b'2'), Some(b' '), _) => {
            Some((952, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'5'), Some(b'3'), Some(b' '), _) => {
            Some((953, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'5'), Some(b'4'), Some(b' '), _) => {
            Some((954, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'5'), Some(b'5'), Some(b' '), _) => {
            Some((955, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'5'), Some(b'6'), Some(b' '), _) => {
            Some((956, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'5'), Some(b'7'), Some(b' '), _) => {
            Some((957, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'5'), Some(b'8'), Some(b' '), _) => {
            Some((958, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'5'), Some(b'9'), Some(b' '), _) => {
            Some((959, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'6'), Some(b'0'), Some(b' '), _) => {
            Some((960, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'6'), Some(b'1'), Some(b' '), _) => {
            Some((961, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'6'), Some(b'2'), Some(b' '), _) => {
            Some((962, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'6'), Some(b'3'), Some(b' '), _) => {
            Some((963, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'6'), Some(b'4'), Some(b' '), _) => {
            Some((964, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'6'), Some(b'5'), Some(b' '), _) => {
            Some((965, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'6'), Some(b'6'), Some(b' '), _) => {
            Some((966, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'6'), Some(b'7'), Some(b' '), _) => {
            Some((967, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'6'), Some(b'8'), Some(b' '), _) => {
            Some((968, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'6'), Some(b'9'), Some(b' '), _) => {
            Some((969, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'7'), Some(b'0'), Some(b' '), _) => {
            Some((970, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'7'), Some(b'1'), Some(b' '), _) => {
            Some((971, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'7'), Some(b'2'), Some(b' '), _) => {
            Some((972, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'7'), Some(b'3'), Some(b' '), _) => {
            Some((973, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'7'), Some(b'4'), Some(b' '), _) => {
            Some((974, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'7'), Some(b'5'), Some(b' '), _) => {
            Some((975, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'7'), Some(b'6'), Some(b' '), _) => {
            Some((976, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'7'), Some(b'7'), Some(b' '), _) => {
            Some((977, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'7'), Some(b'8'), Some(b' '), _) => {
            Some((978, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'7'), Some(b'9'), Some(b' '), _) => {
            Some((979, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'8'), Some(b'0'), Some(b' '), _) => {
            Some((980, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'8'), Some(b'1'), Some(b' '), _) => {
            Some((981, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'8'), Some(b'2'), Some(b' '), _) => {
            Some((982, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'8'), Some(b'3'), Some(b' '), _) => {
            Some((983, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'8'), Some(b'4'), Some(b' '), _) => {
            Some((984, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'8'), Some(b'5'), Some(b' '), _) => {
            Some((985, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'8'), Some(b'6'), Some(b' '), _) => {
            Some((986, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'8'), Some(b'7'), Some(b' '), _) => {
            Some((987, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'8'), Some(b'8'), Some(b' '), _) => {
            Some((988, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'8'), Some(b'9'), Some(b' '), _) => {
            Some((989, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'9'), Some(b'0'), Some(b' '), _) => {
            Some((990, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'9'), Some(b'1'), Some(b' '), _) => {
            Some((991, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'9'), Some(b'2'), Some(b' '), _) => {
            Some((992, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'9'), Some(b'3'), Some(b' '), _) => {
            Some((993, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'9'), Some(b'4'), Some(b' '), _) => {
            Some((994, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'9'), Some(b'5'), Some(b' '), _) => {
            Some((995, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'9'), Some(b'6'), Some(b' '), _) => {
            Some((996, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'9'), Some(b'7'), Some(b' '), _) => {
            Some((997, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'9'), Some(b'8'), Some(b' '), _) => {
            Some((998, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'9'), Some(b'9'), Some(b'9'), Some(b' '), _) => {
            Some((999, unsafe { buffer.get_unchecked(4..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'0'), Some(b'0'), Some(b' ')) => {
            Some((1000, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'0'), Some(b'1'), Some(b' ')) => {
            Some((1001, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'0'), Some(b'2'), Some(b' ')) => {
            Some((1002, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'0'), Some(b'3'), Some(b' ')) => {
            Some((1003, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'0'), Some(b'4'), Some(b' ')) => {
            Some((1004, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'0'), Some(b'5'), Some(b' ')) => {
            Some((1005, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'0'), Some(b'6'), Some(b' ')) => {
            Some((1006, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'0'), Some(b'7'), Some(b' ')) => {
            Some((1007, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'0'), Some(b'8'), Some(b' ')) => {
            Some((1008, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'0'), Some(b'9'), Some(b' ')) => {
            Some((1009, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'1'), Some(b'0'), Some(b' ')) => {
            Some((1010, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'1'), Some(b'1'), Some(b' ')) => {
            Some((1011, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'1'), Some(b'2'), Some(b' ')) => {
            Some((1012, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'1'), Some(b'3'), Some(b' ')) => {
            Some((1013, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'1'), Some(b'4'), Some(b' ')) => {
            Some((1014, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'1'), Some(b'5'), Some(b' ')) => {
            Some((1015, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'1'), Some(b'6'), Some(b' ')) => {
            Some((1016, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'1'), Some(b'7'), Some(b' ')) => {
            Some((1017, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'1'), Some(b'8'), Some(b' ')) => {
            Some((1018, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'1'), Some(b'9'), Some(b' ')) => {
            Some((1019, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'2'), Some(b'0'), Some(b' ')) => {
            Some((1020, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'2'), Some(b'1'), Some(b' ')) => {
            Some((1021, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'2'), Some(b'2'), Some(b' ')) => {
            Some((1022, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'2'), Some(b'3'), Some(b' ')) => {
            Some((1023, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'2'), Some(b'4'), Some(b' ')) => {
            Some((1024, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'2'), Some(b'5'), Some(b' ')) => {
            Some((1025, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'2'), Some(b'6'), Some(b' ')) => {
            Some((1026, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'2'), Some(b'7'), Some(b' ')) => {
            Some((1027, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'2'), Some(b'8'), Some(b' ')) => {
            Some((1028, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'2'), Some(b'9'), Some(b' ')) => {
            Some((1029, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'3'), Some(b'0'), Some(b' ')) => {
            Some((1030, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'3'), Some(b'1'), Some(b' ')) => {
            Some((1031, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'3'), Some(b'2'), Some(b' ')) => {
            Some((1032, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'3'), Some(b'3'), Some(b' ')) => {
            Some((1033, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'3'), Some(b'4'), Some(b' ')) => {
            Some((1034, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'3'), Some(b'5'), Some(b' ')) => {
            Some((1035, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'3'), Some(b'6'), Some(b' ')) => {
            Some((1036, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'3'), Some(b'7'), Some(b' ')) => {
            Some((1037, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'3'), Some(b'8'), Some(b' ')) => {
            Some((1038, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'3'), Some(b'9'), Some(b' ')) => {
            Some((1039, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'4'), Some(b'0'), Some(b' ')) => {
            Some((1040, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'4'), Some(b'1'), Some(b' ')) => {
            Some((1041, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'4'), Some(b'2'), Some(b' ')) => {
            Some((1042, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'4'), Some(b'3'), Some(b' ')) => {
            Some((1043, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'4'), Some(b'4'), Some(b' ')) => {
            Some((1044, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'4'), Some(b'5'), Some(b' ')) => {
            Some((1045, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'4'), Some(b'6'), Some(b' ')) => {
            Some((1046, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'4'), Some(b'7'), Some(b' ')) => {
            Some((1047, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'4'), Some(b'8'), Some(b' ')) => {
            Some((1048, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'4'), Some(b'9'), Some(b' ')) => {
            Some((1049, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'5'), Some(b'0'), Some(b' ')) => {
            Some((1050, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'5'), Some(b'1'), Some(b' ')) => {
            Some((1051, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'5'), Some(b'2'), Some(b' ')) => {
            Some((1052, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'5'), Some(b'3'), Some(b' ')) => {
            Some((1053, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'5'), Some(b'4'), Some(b' ')) => {
            Some((1054, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'5'), Some(b'5'), Some(b' ')) => {
            Some((1055, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'5'), Some(b'6'), Some(b' ')) => {
            Some((1056, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'5'), Some(b'7'), Some(b' ')) => {
            Some((1057, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'5'), Some(b'8'), Some(b' ')) => {
            Some((1058, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'5'), Some(b'9'), Some(b' ')) => {
            Some((1059, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'6'), Some(b'0'), Some(b' ')) => {
            Some((1060, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'6'), Some(b'1'), Some(b' ')) => {
            Some((1061, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'6'), Some(b'2'), Some(b' ')) => {
            Some((1062, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'6'), Some(b'3'), Some(b' ')) => {
            Some((1063, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'6'), Some(b'4'), Some(b' ')) => {
            Some((1064, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'6'), Some(b'5'), Some(b' ')) => {
            Some((1065, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'6'), Some(b'6'), Some(b' ')) => {
            Some((1066, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'6'), Some(b'7'), Some(b' ')) => {
            Some((1067, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'6'), Some(b'8'), Some(b' ')) => {
            Some((1068, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'6'), Some(b'9'), Some(b' ')) => {
            Some((1069, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'7'), Some(b'0'), Some(b' ')) => {
            Some((1070, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'7'), Some(b'1'), Some(b' ')) => {
            Some((1071, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'7'), Some(b'2'), Some(b' ')) => {
            Some((1072, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'7'), Some(b'3'), Some(b' ')) => {
            Some((1073, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'7'), Some(b'4'), Some(b' ')) => {
            Some((1074, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'7'), Some(b'5'), Some(b' ')) => {
            Some((1075, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'7'), Some(b'6'), Some(b' ')) => {
            Some((1076, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'7'), Some(b'7'), Some(b' ')) => {
            Some((1077, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'7'), Some(b'8'), Some(b' ')) => {
            Some((1078, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'7'), Some(b'9'), Some(b' ')) => {
            Some((1079, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'8'), Some(b'0'), Some(b' ')) => {
            Some((1080, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'8'), Some(b'1'), Some(b' ')) => {
            Some((1081, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'8'), Some(b'2'), Some(b' ')) => {
            Some((1082, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'8'), Some(b'3'), Some(b' ')) => {
            Some((1083, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'8'), Some(b'4'), Some(b' ')) => {
            Some((1084, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'8'), Some(b'5'), Some(b' ')) => {
            Some((1085, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'8'), Some(b'6'), Some(b' ')) => {
            Some((1086, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'8'), Some(b'7'), Some(b' ')) => {
            Some((1087, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'8'), Some(b'8'), Some(b' ')) => {
            Some((1088, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'8'), Some(b'9'), Some(b' ')) => {
            Some((1089, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'9'), Some(b'0'), Some(b' ')) => {
            Some((1090, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'9'), Some(b'1'), Some(b' ')) => {
            Some((1091, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'9'), Some(b'2'), Some(b' ')) => {
            Some((1092, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'9'), Some(b'3'), Some(b' ')) => {
            Some((1093, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'9'), Some(b'4'), Some(b' ')) => {
            Some((1094, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'9'), Some(b'5'), Some(b' ')) => {
            Some((1095, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'9'), Some(b'6'), Some(b' ')) => {
            Some((1096, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'9'), Some(b'7'), Some(b' ')) => {
            Some((1097, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'9'), Some(b'8'), Some(b' ')) => {
            Some((1098, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'0'), Some(b'9'), Some(b'9'), Some(b' ')) => {
            Some((1099, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'0'), Some(b'0'), Some(b' ')) => {
            Some((1100, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'0'), Some(b'1'), Some(b' ')) => {
            Some((1101, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'0'), Some(b'2'), Some(b' ')) => {
            Some((1102, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'0'), Some(b'3'), Some(b' ')) => {
            Some((1103, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'0'), Some(b'4'), Some(b' ')) => {
            Some((1104, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'0'), Some(b'5'), Some(b' ')) => {
            Some((1105, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'0'), Some(b'6'), Some(b' ')) => {
            Some((1106, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'0'), Some(b'7'), Some(b' ')) => {
            Some((1107, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'0'), Some(b'8'), Some(b' ')) => {
            Some((1108, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'0'), Some(b'9'), Some(b' ')) => {
            Some((1109, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'1'), Some(b'0'), Some(b' ')) => {
            Some((1110, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'1'), Some(b'1'), Some(b' ')) => {
            Some((1111, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'1'), Some(b'2'), Some(b' ')) => {
            Some((1112, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'1'), Some(b'3'), Some(b' ')) => {
            Some((1113, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'1'), Some(b'4'), Some(b' ')) => {
            Some((1114, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'1'), Some(b'5'), Some(b' ')) => {
            Some((1115, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'1'), Some(b'6'), Some(b' ')) => {
            Some((1116, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'1'), Some(b'7'), Some(b' ')) => {
            Some((1117, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'1'), Some(b'8'), Some(b' ')) => {
            Some((1118, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'1'), Some(b'9'), Some(b' ')) => {
            Some((1119, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'2'), Some(b'0'), Some(b' ')) => {
            Some((1120, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'2'), Some(b'1'), Some(b' ')) => {
            Some((1121, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'2'), Some(b'2'), Some(b' ')) => {
            Some((1122, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'2'), Some(b'3'), Some(b' ')) => {
            Some((1123, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'2'), Some(b'4'), Some(b' ')) => {
            Some((1124, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'2'), Some(b'5'), Some(b' ')) => {
            Some((1125, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'2'), Some(b'6'), Some(b' ')) => {
            Some((1126, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'2'), Some(b'7'), Some(b' ')) => {
            Some((1127, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'2'), Some(b'8'), Some(b' ')) => {
            Some((1128, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'2'), Some(b'9'), Some(b' ')) => {
            Some((1129, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'3'), Some(b'0'), Some(b' ')) => {
            Some((1130, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'3'), Some(b'1'), Some(b' ')) => {
            Some((1131, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'3'), Some(b'2'), Some(b' ')) => {
            Some((1132, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'3'), Some(b'3'), Some(b' ')) => {
            Some((1133, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'3'), Some(b'4'), Some(b' ')) => {
            Some((1134, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'3'), Some(b'5'), Some(b' ')) => {
            Some((1135, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'3'), Some(b'6'), Some(b' ')) => {
            Some((1136, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'3'), Some(b'7'), Some(b' ')) => {
            Some((1137, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'3'), Some(b'8'), Some(b' ')) => {
            Some((1138, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'3'), Some(b'9'), Some(b' ')) => {
            Some((1139, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'4'), Some(b'0'), Some(b' ')) => {
            Some((1140, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'4'), Some(b'1'), Some(b' ')) => {
            Some((1141, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'4'), Some(b'2'), Some(b' ')) => {
            Some((1142, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'4'), Some(b'3'), Some(b' ')) => {
            Some((1143, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'4'), Some(b'4'), Some(b' ')) => {
            Some((1144, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'4'), Some(b'5'), Some(b' ')) => {
            Some((1145, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'4'), Some(b'6'), Some(b' ')) => {
            Some((1146, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'4'), Some(b'7'), Some(b' ')) => {
            Some((1147, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'4'), Some(b'8'), Some(b' ')) => {
            Some((1148, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'4'), Some(b'9'), Some(b' ')) => {
            Some((1149, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'5'), Some(b'0'), Some(b' ')) => {
            Some((1150, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'5'), Some(b'1'), Some(b' ')) => {
            Some((1151, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'5'), Some(b'2'), Some(b' ')) => {
            Some((1152, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'5'), Some(b'3'), Some(b' ')) => {
            Some((1153, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'5'), Some(b'4'), Some(b' ')) => {
            Some((1154, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'5'), Some(b'5'), Some(b' ')) => {
            Some((1155, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'5'), Some(b'6'), Some(b' ')) => {
            Some((1156, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'5'), Some(b'7'), Some(b' ')) => {
            Some((1157, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'5'), Some(b'8'), Some(b' ')) => {
            Some((1158, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'5'), Some(b'9'), Some(b' ')) => {
            Some((1159, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'6'), Some(b'0'), Some(b' ')) => {
            Some((1160, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'6'), Some(b'1'), Some(b' ')) => {
            Some((1161, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'6'), Some(b'2'), Some(b' ')) => {
            Some((1162, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'6'), Some(b'3'), Some(b' ')) => {
            Some((1163, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'6'), Some(b'4'), Some(b' ')) => {
            Some((1164, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'6'), Some(b'5'), Some(b' ')) => {
            Some((1165, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'6'), Some(b'6'), Some(b' ')) => {
            Some((1166, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'6'), Some(b'7'), Some(b' ')) => {
            Some((1167, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'6'), Some(b'8'), Some(b' ')) => {
            Some((1168, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'6'), Some(b'9'), Some(b' ')) => {
            Some((1169, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'7'), Some(b'0'), Some(b' ')) => {
            Some((1170, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'7'), Some(b'1'), Some(b' ')) => {
            Some((1171, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'7'), Some(b'2'), Some(b' ')) => {
            Some((1172, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'7'), Some(b'3'), Some(b' ')) => {
            Some((1173, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'7'), Some(b'4'), Some(b' ')) => {
            Some((1174, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'7'), Some(b'5'), Some(b' ')) => {
            Some((1175, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'7'), Some(b'6'), Some(b' ')) => {
            Some((1176, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'7'), Some(b'7'), Some(b' ')) => {
            Some((1177, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'7'), Some(b'8'), Some(b' ')) => {
            Some((1178, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'7'), Some(b'9'), Some(b' ')) => {
            Some((1179, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'8'), Some(b'0'), Some(b' ')) => {
            Some((1180, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'8'), Some(b'1'), Some(b' ')) => {
            Some((1181, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'8'), Some(b'2'), Some(b' ')) => {
            Some((1182, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'8'), Some(b'3'), Some(b' ')) => {
            Some((1183, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'8'), Some(b'4'), Some(b' ')) => {
            Some((1184, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'8'), Some(b'5'), Some(b' ')) => {
            Some((1185, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'8'), Some(b'6'), Some(b' ')) => {
            Some((1186, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'8'), Some(b'7'), Some(b' ')) => {
            Some((1187, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'8'), Some(b'8'), Some(b' ')) => {
            Some((1188, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'8'), Some(b'9'), Some(b' ')) => {
            Some((1189, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'9'), Some(b'0'), Some(b' ')) => {
            Some((1190, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'9'), Some(b'1'), Some(b' ')) => {
            Some((1191, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'9'), Some(b'2'), Some(b' ')) => {
            Some((1192, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'9'), Some(b'3'), Some(b' ')) => {
            Some((1193, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'9'), Some(b'4'), Some(b' ')) => {
            Some((1194, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'9'), Some(b'5'), Some(b' ')) => {
            Some((1195, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'9'), Some(b'6'), Some(b' ')) => {
            Some((1196, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'9'), Some(b'7'), Some(b' ')) => {
            Some((1197, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'9'), Some(b'8'), Some(b' ')) => {
            Some((1198, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'1'), Some(b'9'), Some(b'9'), Some(b' ')) => {
            Some((1199, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'0'), Some(b'0'), Some(b' ')) => {
            Some((1200, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'0'), Some(b'1'), Some(b' ')) => {
            Some((1201, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'0'), Some(b'2'), Some(b' ')) => {
            Some((1202, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'0'), Some(b'3'), Some(b' ')) => {
            Some((1203, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'0'), Some(b'4'), Some(b' ')) => {
            Some((1204, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'0'), Some(b'5'), Some(b' ')) => {
            Some((1205, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'0'), Some(b'6'), Some(b' ')) => {
            Some((1206, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'0'), Some(b'7'), Some(b' ')) => {
            Some((1207, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'0'), Some(b'8'), Some(b' ')) => {
            Some((1208, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'0'), Some(b'9'), Some(b' ')) => {
            Some((1209, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'1'), Some(b'0'), Some(b' ')) => {
            Some((1210, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'1'), Some(b'1'), Some(b' ')) => {
            Some((1211, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'1'), Some(b'2'), Some(b' ')) => {
            Some((1212, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'1'), Some(b'3'), Some(b' ')) => {
            Some((1213, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'1'), Some(b'4'), Some(b' ')) => {
            Some((1214, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'1'), Some(b'5'), Some(b' ')) => {
            Some((1215, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'1'), Some(b'6'), Some(b' ')) => {
            Some((1216, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'1'), Some(b'7'), Some(b' ')) => {
            Some((1217, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'1'), Some(b'8'), Some(b' ')) => {
            Some((1218, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'1'), Some(b'9'), Some(b' ')) => {
            Some((1219, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'2'), Some(b'0'), Some(b' ')) => {
            Some((1220, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'2'), Some(b'1'), Some(b' ')) => {
            Some((1221, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'2'), Some(b'2'), Some(b' ')) => {
            Some((1222, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'2'), Some(b'3'), Some(b' ')) => {
            Some((1223, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'2'), Some(b'4'), Some(b' ')) => {
            Some((1224, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'2'), Some(b'5'), Some(b' ')) => {
            Some((1225, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'2'), Some(b'6'), Some(b' ')) => {
            Some((1226, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'2'), Some(b'7'), Some(b' ')) => {
            Some((1227, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'2'), Some(b'8'), Some(b' ')) => {
            Some((1228, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'2'), Some(b'9'), Some(b' ')) => {
            Some((1229, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'3'), Some(b'0'), Some(b' ')) => {
            Some((1230, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'3'), Some(b'1'), Some(b' ')) => {
            Some((1231, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'3'), Some(b'2'), Some(b' ')) => {
            Some((1232, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'3'), Some(b'3'), Some(b' ')) => {
            Some((1233, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'3'), Some(b'4'), Some(b' ')) => {
            Some((1234, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'3'), Some(b'5'), Some(b' ')) => {
            Some((1235, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'3'), Some(b'6'), Some(b' ')) => {
            Some((1236, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'3'), Some(b'7'), Some(b' ')) => {
            Some((1237, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'3'), Some(b'8'), Some(b' ')) => {
            Some((1238, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'3'), Some(b'9'), Some(b' ')) => {
            Some((1239, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'4'), Some(b'0'), Some(b' ')) => {
            Some((1240, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'4'), Some(b'1'), Some(b' ')) => {
            Some((1241, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'4'), Some(b'2'), Some(b' ')) => {
            Some((1242, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'4'), Some(b'3'), Some(b' ')) => {
            Some((1243, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'4'), Some(b'4'), Some(b' ')) => {
            Some((1244, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'4'), Some(b'5'), Some(b' ')) => {
            Some((1245, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'4'), Some(b'6'), Some(b' ')) => {
            Some((1246, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'4'), Some(b'7'), Some(b' ')) => {
            Some((1247, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'4'), Some(b'8'), Some(b' ')) => {
            Some((1248, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'4'), Some(b'9'), Some(b' ')) => {
            Some((1249, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'5'), Some(b'0'), Some(b' ')) => {
            Some((1250, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'5'), Some(b'1'), Some(b' ')) => {
            Some((1251, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'5'), Some(b'2'), Some(b' ')) => {
            Some((1252, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'5'), Some(b'3'), Some(b' ')) => {
            Some((1253, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'5'), Some(b'4'), Some(b' ')) => {
            Some((1254, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'5'), Some(b'5'), Some(b' ')) => {
            Some((1255, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'5'), Some(b'6'), Some(b' ')) => {
            Some((1256, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'5'), Some(b'7'), Some(b' ')) => {
            Some((1257, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'5'), Some(b'8'), Some(b' ')) => {
            Some((1258, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'5'), Some(b'9'), Some(b' ')) => {
            Some((1259, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'6'), Some(b'0'), Some(b' ')) => {
            Some((1260, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'6'), Some(b'1'), Some(b' ')) => {
            Some((1261, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'6'), Some(b'2'), Some(b' ')) => {
            Some((1262, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'6'), Some(b'3'), Some(b' ')) => {
            Some((1263, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'6'), Some(b'4'), Some(b' ')) => {
            Some((1264, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'6'), Some(b'5'), Some(b' ')) => {
            Some((1265, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'6'), Some(b'6'), Some(b' ')) => {
            Some((1266, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'6'), Some(b'7'), Some(b' ')) => {
            Some((1267, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'6'), Some(b'8'), Some(b' ')) => {
            Some((1268, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'6'), Some(b'9'), Some(b' ')) => {
            Some((1269, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'7'), Some(b'0'), Some(b' ')) => {
            Some((1270, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'7'), Some(b'1'), Some(b' ')) => {
            Some((1271, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'7'), Some(b'2'), Some(b' ')) => {
            Some((1272, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'7'), Some(b'3'), Some(b' ')) => {
            Some((1273, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'7'), Some(b'4'), Some(b' ')) => {
            Some((1274, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'7'), Some(b'5'), Some(b' ')) => {
            Some((1275, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'7'), Some(b'6'), Some(b' ')) => {
            Some((1276, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'7'), Some(b'7'), Some(b' ')) => {
            Some((1277, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'7'), Some(b'8'), Some(b' ')) => {
            Some((1278, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'7'), Some(b'9'), Some(b' ')) => {
            Some((1279, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'8'), Some(b'0'), Some(b' ')) => {
            Some((1280, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'8'), Some(b'1'), Some(b' ')) => {
            Some((1281, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'8'), Some(b'2'), Some(b' ')) => {
            Some((1282, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'8'), Some(b'3'), Some(b' ')) => {
            Some((1283, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'8'), Some(b'4'), Some(b' ')) => {
            Some((1284, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'8'), Some(b'5'), Some(b' ')) => {
            Some((1285, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'8'), Some(b'6'), Some(b' ')) => {
            Some((1286, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'8'), Some(b'7'), Some(b' ')) => {
            Some((1287, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'8'), Some(b'8'), Some(b' ')) => {
            Some((1288, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'8'), Some(b'9'), Some(b' ')) => {
            Some((1289, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'9'), Some(b'0'), Some(b' ')) => {
            Some((1290, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'9'), Some(b'1'), Some(b' ')) => {
            Some((1291, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'9'), Some(b'2'), Some(b' ')) => {
            Some((1292, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'9'), Some(b'3'), Some(b' ')) => {
            Some((1293, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'9'), Some(b'4'), Some(b' ')) => {
            Some((1294, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'9'), Some(b'5'), Some(b' ')) => {
            Some((1295, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'9'), Some(b'6'), Some(b' ')) => {
            Some((1296, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'9'), Some(b'7'), Some(b' ')) => {
            Some((1297, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'9'), Some(b'8'), Some(b' ')) => {
            Some((1298, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'2'), Some(b'9'), Some(b'9'), Some(b' ')) => {
            Some((1299, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'0'), Some(b'0'), Some(b' ')) => {
            Some((1300, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'0'), Some(b'1'), Some(b' ')) => {
            Some((1301, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'0'), Some(b'2'), Some(b' ')) => {
            Some((1302, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'0'), Some(b'3'), Some(b' ')) => {
            Some((1303, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'0'), Some(b'4'), Some(b' ')) => {
            Some((1304, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'0'), Some(b'5'), Some(b' ')) => {
            Some((1305, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'0'), Some(b'6'), Some(b' ')) => {
            Some((1306, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'0'), Some(b'7'), Some(b' ')) => {
            Some((1307, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'0'), Some(b'8'), Some(b' ')) => {
            Some((1308, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'0'), Some(b'9'), Some(b' ')) => {
            Some((1309, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'1'), Some(b'0'), Some(b' ')) => {
            Some((1310, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'1'), Some(b'1'), Some(b' ')) => {
            Some((1311, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'1'), Some(b'2'), Some(b' ')) => {
            Some((1312, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'1'), Some(b'3'), Some(b' ')) => {
            Some((1313, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'1'), Some(b'4'), Some(b' ')) => {
            Some((1314, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'1'), Some(b'5'), Some(b' ')) => {
            Some((1315, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'1'), Some(b'6'), Some(b' ')) => {
            Some((1316, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'1'), Some(b'7'), Some(b' ')) => {
            Some((1317, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'1'), Some(b'8'), Some(b' ')) => {
            Some((1318, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'1'), Some(b'9'), Some(b' ')) => {
            Some((1319, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'2'), Some(b'0'), Some(b' ')) => {
            Some((1320, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'2'), Some(b'1'), Some(b' ')) => {
            Some((1321, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'2'), Some(b'2'), Some(b' ')) => {
            Some((1322, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'2'), Some(b'3'), Some(b' ')) => {
            Some((1323, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'2'), Some(b'4'), Some(b' ')) => {
            Some((1324, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'2'), Some(b'5'), Some(b' ')) => {
            Some((1325, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'2'), Some(b'6'), Some(b' ')) => {
            Some((1326, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'2'), Some(b'7'), Some(b' ')) => {
            Some((1327, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'2'), Some(b'8'), Some(b' ')) => {
            Some((1328, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'2'), Some(b'9'), Some(b' ')) => {
            Some((1329, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'3'), Some(b'0'), Some(b' ')) => {
            Some((1330, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'3'), Some(b'1'), Some(b' ')) => {
            Some((1331, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'3'), Some(b'2'), Some(b' ')) => {
            Some((1332, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'3'), Some(b'3'), Some(b' ')) => {
            Some((1333, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'3'), Some(b'4'), Some(b' ')) => {
            Some((1334, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'3'), Some(b'5'), Some(b' ')) => {
            Some((1335, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'3'), Some(b'6'), Some(b' ')) => {
            Some((1336, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'3'), Some(b'7'), Some(b' ')) => {
            Some((1337, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'3'), Some(b'8'), Some(b' ')) => {
            Some((1338, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'3'), Some(b'9'), Some(b' ')) => {
            Some((1339, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'4'), Some(b'0'), Some(b' ')) => {
            Some((1340, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'4'), Some(b'1'), Some(b' ')) => {
            Some((1341, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'4'), Some(b'2'), Some(b' ')) => {
            Some((1342, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'4'), Some(b'3'), Some(b' ')) => {
            Some((1343, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'4'), Some(b'4'), Some(b' ')) => {
            Some((1344, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'4'), Some(b'5'), Some(b' ')) => {
            Some((1345, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'4'), Some(b'6'), Some(b' ')) => {
            Some((1346, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'4'), Some(b'7'), Some(b' ')) => {
            Some((1347, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'4'), Some(b'8'), Some(b' ')) => {
            Some((1348, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'4'), Some(b'9'), Some(b' ')) => {
            Some((1349, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'5'), Some(b'0'), Some(b' ')) => {
            Some((1350, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'5'), Some(b'1'), Some(b' ')) => {
            Some((1351, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'5'), Some(b'2'), Some(b' ')) => {
            Some((1352, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'5'), Some(b'3'), Some(b' ')) => {
            Some((1353, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'5'), Some(b'4'), Some(b' ')) => {
            Some((1354, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'5'), Some(b'5'), Some(b' ')) => {
            Some((1355, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'5'), Some(b'6'), Some(b' ')) => {
            Some((1356, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'5'), Some(b'7'), Some(b' ')) => {
            Some((1357, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'5'), Some(b'8'), Some(b' ')) => {
            Some((1358, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'5'), Some(b'9'), Some(b' ')) => {
            Some((1359, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'6'), Some(b'0'), Some(b' ')) => {
            Some((1360, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'6'), Some(b'1'), Some(b' ')) => {
            Some((1361, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'6'), Some(b'2'), Some(b' ')) => {
            Some((1362, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'6'), Some(b'3'), Some(b' ')) => {
            Some((1363, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'6'), Some(b'4'), Some(b' ')) => {
            Some((1364, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'6'), Some(b'5'), Some(b' ')) => {
            Some((1365, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'6'), Some(b'6'), Some(b' ')) => {
            Some((1366, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'6'), Some(b'7'), Some(b' ')) => {
            Some((1367, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'6'), Some(b'8'), Some(b' ')) => {
            Some((1368, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'6'), Some(b'9'), Some(b' ')) => {
            Some((1369, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'7'), Some(b'0'), Some(b' ')) => {
            Some((1370, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'7'), Some(b'1'), Some(b' ')) => {
            Some((1371, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'7'), Some(b'2'), Some(b' ')) => {
            Some((1372, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'7'), Some(b'3'), Some(b' ')) => {
            Some((1373, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'7'), Some(b'4'), Some(b' ')) => {
            Some((1374, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'7'), Some(b'5'), Some(b' ')) => {
            Some((1375, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'7'), Some(b'6'), Some(b' ')) => {
            Some((1376, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'7'), Some(b'7'), Some(b' ')) => {
            Some((1377, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'7'), Some(b'8'), Some(b' ')) => {
            Some((1378, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'7'), Some(b'9'), Some(b' ')) => {
            Some((1379, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'8'), Some(b'0'), Some(b' ')) => {
            Some((1380, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'8'), Some(b'1'), Some(b' ')) => {
            Some((1381, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'8'), Some(b'2'), Some(b' ')) => {
            Some((1382, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'8'), Some(b'3'), Some(b' ')) => {
            Some((1383, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'8'), Some(b'4'), Some(b' ')) => {
            Some((1384, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'8'), Some(b'5'), Some(b' ')) => {
            Some((1385, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'8'), Some(b'6'), Some(b' ')) => {
            Some((1386, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'8'), Some(b'7'), Some(b' ')) => {
            Some((1387, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'8'), Some(b'8'), Some(b' ')) => {
            Some((1388, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'8'), Some(b'9'), Some(b' ')) => {
            Some((1389, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'9'), Some(b'0'), Some(b' ')) => {
            Some((1390, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'9'), Some(b'1'), Some(b' ')) => {
            Some((1391, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'9'), Some(b'2'), Some(b' ')) => {
            Some((1392, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'9'), Some(b'3'), Some(b' ')) => {
            Some((1393, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'9'), Some(b'4'), Some(b' ')) => {
            Some((1394, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'9'), Some(b'5'), Some(b' ')) => {
            Some((1395, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'9'), Some(b'6'), Some(b' ')) => {
            Some((1396, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'9'), Some(b'7'), Some(b' ')) => {
            Some((1397, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'9'), Some(b'8'), Some(b' ')) => {
            Some((1398, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'3'), Some(b'9'), Some(b'9'), Some(b' ')) => {
            Some((1399, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'0'), Some(b'0'), Some(b' ')) => {
            Some((1400, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'0'), Some(b'1'), Some(b' ')) => {
            Some((1401, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'0'), Some(b'2'), Some(b' ')) => {
            Some((1402, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'0'), Some(b'3'), Some(b' ')) => {
            Some((1403, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'0'), Some(b'4'), Some(b' ')) => {
            Some((1404, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'0'), Some(b'5'), Some(b' ')) => {
            Some((1405, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'0'), Some(b'6'), Some(b' ')) => {
            Some((1406, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'0'), Some(b'7'), Some(b' ')) => {
            Some((1407, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'0'), Some(b'8'), Some(b' ')) => {
            Some((1408, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'0'), Some(b'9'), Some(b' ')) => {
            Some((1409, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'1'), Some(b'0'), Some(b' ')) => {
            Some((1410, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'1'), Some(b'1'), Some(b' ')) => {
            Some((1411, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'1'), Some(b'2'), Some(b' ')) => {
            Some((1412, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'1'), Some(b'3'), Some(b' ')) => {
            Some((1413, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'1'), Some(b'4'), Some(b' ')) => {
            Some((1414, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'1'), Some(b'5'), Some(b' ')) => {
            Some((1415, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'1'), Some(b'6'), Some(b' ')) => {
            Some((1416, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'1'), Some(b'7'), Some(b' ')) => {
            Some((1417, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'1'), Some(b'8'), Some(b' ')) => {
            Some((1418, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'1'), Some(b'9'), Some(b' ')) => {
            Some((1419, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'2'), Some(b'0'), Some(b' ')) => {
            Some((1420, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'2'), Some(b'1'), Some(b' ')) => {
            Some((1421, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'2'), Some(b'2'), Some(b' ')) => {
            Some((1422, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'2'), Some(b'3'), Some(b' ')) => {
            Some((1423, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'2'), Some(b'4'), Some(b' ')) => {
            Some((1424, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'2'), Some(b'5'), Some(b' ')) => {
            Some((1425, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'2'), Some(b'6'), Some(b' ')) => {
            Some((1426, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'2'), Some(b'7'), Some(b' ')) => {
            Some((1427, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'2'), Some(b'8'), Some(b' ')) => {
            Some((1428, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'2'), Some(b'9'), Some(b' ')) => {
            Some((1429, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'3'), Some(b'0'), Some(b' ')) => {
            Some((1430, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'3'), Some(b'1'), Some(b' ')) => {
            Some((1431, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'3'), Some(b'2'), Some(b' ')) => {
            Some((1432, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'3'), Some(b'3'), Some(b' ')) => {
            Some((1433, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'3'), Some(b'4'), Some(b' ')) => {
            Some((1434, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'3'), Some(b'5'), Some(b' ')) => {
            Some((1435, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'3'), Some(b'6'), Some(b' ')) => {
            Some((1436, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'3'), Some(b'7'), Some(b' ')) => {
            Some((1437, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'3'), Some(b'8'), Some(b' ')) => {
            Some((1438, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'3'), Some(b'9'), Some(b' ')) => {
            Some((1439, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'4'), Some(b'0'), Some(b' ')) => {
            Some((1440, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'4'), Some(b'1'), Some(b' ')) => {
            Some((1441, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'4'), Some(b'2'), Some(b' ')) => {
            Some((1442, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'4'), Some(b'3'), Some(b' ')) => {
            Some((1443, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'4'), Some(b'4'), Some(b' ')) => {
            Some((1444, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'4'), Some(b'5'), Some(b' ')) => {
            Some((1445, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'4'), Some(b'6'), Some(b' ')) => {
            Some((1446, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'4'), Some(b'7'), Some(b' ')) => {
            Some((1447, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'4'), Some(b'8'), Some(b' ')) => {
            Some((1448, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'4'), Some(b'9'), Some(b' ')) => {
            Some((1449, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'5'), Some(b'0'), Some(b' ')) => {
            Some((1450, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'5'), Some(b'1'), Some(b' ')) => {
            Some((1451, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'5'), Some(b'2'), Some(b' ')) => {
            Some((1452, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'5'), Some(b'3'), Some(b' ')) => {
            Some((1453, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'5'), Some(b'4'), Some(b' ')) => {
            Some((1454, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'5'), Some(b'5'), Some(b' ')) => {
            Some((1455, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'5'), Some(b'6'), Some(b' ')) => {
            Some((1456, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'5'), Some(b'7'), Some(b' ')) => {
            Some((1457, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'5'), Some(b'8'), Some(b' ')) => {
            Some((1458, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'5'), Some(b'9'), Some(b' ')) => {
            Some((1459, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'6'), Some(b'0'), Some(b' ')) => {
            Some((1460, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'6'), Some(b'1'), Some(b' ')) => {
            Some((1461, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'6'), Some(b'2'), Some(b' ')) => {
            Some((1462, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'6'), Some(b'3'), Some(b' ')) => {
            Some((1463, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'6'), Some(b'4'), Some(b' ')) => {
            Some((1464, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'6'), Some(b'5'), Some(b' ')) => {
            Some((1465, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'6'), Some(b'6'), Some(b' ')) => {
            Some((1466, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'6'), Some(b'7'), Some(b' ')) => {
            Some((1467, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'6'), Some(b'8'), Some(b' ')) => {
            Some((1468, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'6'), Some(b'9'), Some(b' ')) => {
            Some((1469, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'7'), Some(b'0'), Some(b' ')) => {
            Some((1470, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'7'), Some(b'1'), Some(b' ')) => {
            Some((1471, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'7'), Some(b'2'), Some(b' ')) => {
            Some((1472, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'7'), Some(b'3'), Some(b' ')) => {
            Some((1473, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'7'), Some(b'4'), Some(b' ')) => {
            Some((1474, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'7'), Some(b'5'), Some(b' ')) => {
            Some((1475, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'7'), Some(b'6'), Some(b' ')) => {
            Some((1476, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'7'), Some(b'7'), Some(b' ')) => {
            Some((1477, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'7'), Some(b'8'), Some(b' ')) => {
            Some((1478, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'7'), Some(b'9'), Some(b' ')) => {
            Some((1479, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'8'), Some(b'0'), Some(b' ')) => {
            Some((1480, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'8'), Some(b'1'), Some(b' ')) => {
            Some((1481, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'8'), Some(b'2'), Some(b' ')) => {
            Some((1482, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'8'), Some(b'3'), Some(b' ')) => {
            Some((1483, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'8'), Some(b'4'), Some(b' ')) => {
            Some((1484, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'8'), Some(b'5'), Some(b' ')) => {
            Some((1485, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'8'), Some(b'6'), Some(b' ')) => {
            Some((1486, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'8'), Some(b'7'), Some(b' ')) => {
            Some((1487, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'8'), Some(b'8'), Some(b' ')) => {
            Some((1488, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'8'), Some(b'9'), Some(b' ')) => {
            Some((1489, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'9'), Some(b'0'), Some(b' ')) => {
            Some((1490, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'9'), Some(b'1'), Some(b' ')) => {
            Some((1491, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'9'), Some(b'2'), Some(b' ')) => {
            Some((1492, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'9'), Some(b'3'), Some(b' ')) => {
            Some((1493, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'9'), Some(b'4'), Some(b' ')) => {
            Some((1494, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'9'), Some(b'5'), Some(b' ')) => {
            Some((1495, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'9'), Some(b'6'), Some(b' ')) => {
            Some((1496, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'9'), Some(b'7'), Some(b' ')) => {
            Some((1497, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'9'), Some(b'8'), Some(b' ')) => {
            Some((1498, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'4'), Some(b'9'), Some(b'9'), Some(b' ')) => {
            Some((1499, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'0'), Some(b'0'), Some(b' ')) => {
            Some((1500, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'0'), Some(b'1'), Some(b' ')) => {
            Some((1501, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'0'), Some(b'2'), Some(b' ')) => {
            Some((1502, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'0'), Some(b'3'), Some(b' ')) => {
            Some((1503, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'0'), Some(b'4'), Some(b' ')) => {
            Some((1504, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'0'), Some(b'5'), Some(b' ')) => {
            Some((1505, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'0'), Some(b'6'), Some(b' ')) => {
            Some((1506, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'0'), Some(b'7'), Some(b' ')) => {
            Some((1507, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'0'), Some(b'8'), Some(b' ')) => {
            Some((1508, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'0'), Some(b'9'), Some(b' ')) => {
            Some((1509, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'1'), Some(b'0'), Some(b' ')) => {
            Some((1510, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'1'), Some(b'1'), Some(b' ')) => {
            Some((1511, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'1'), Some(b'2'), Some(b' ')) => {
            Some((1512, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'1'), Some(b'3'), Some(b' ')) => {
            Some((1513, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'1'), Some(b'4'), Some(b' ')) => {
            Some((1514, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'1'), Some(b'5'), Some(b' ')) => {
            Some((1515, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'1'), Some(b'6'), Some(b' ')) => {
            Some((1516, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'1'), Some(b'7'), Some(b' ')) => {
            Some((1517, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'1'), Some(b'8'), Some(b' ')) => {
            Some((1518, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'1'), Some(b'9'), Some(b' ')) => {
            Some((1519, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'2'), Some(b'0'), Some(b' ')) => {
            Some((1520, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'2'), Some(b'1'), Some(b' ')) => {
            Some((1521, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'2'), Some(b'2'), Some(b' ')) => {
            Some((1522, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'2'), Some(b'3'), Some(b' ')) => {
            Some((1523, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'2'), Some(b'4'), Some(b' ')) => {
            Some((1524, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'2'), Some(b'5'), Some(b' ')) => {
            Some((1525, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'2'), Some(b'6'), Some(b' ')) => {
            Some((1526, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'2'), Some(b'7'), Some(b' ')) => {
            Some((1527, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'2'), Some(b'8'), Some(b' ')) => {
            Some((1528, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'2'), Some(b'9'), Some(b' ')) => {
            Some((1529, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'3'), Some(b'0'), Some(b' ')) => {
            Some((1530, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'3'), Some(b'1'), Some(b' ')) => {
            Some((1531, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'3'), Some(b'2'), Some(b' ')) => {
            Some((1532, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'3'), Some(b'3'), Some(b' ')) => {
            Some((1533, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'3'), Some(b'4'), Some(b' ')) => {
            Some((1534, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'3'), Some(b'5'), Some(b' ')) => {
            Some((1535, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'3'), Some(b'6'), Some(b' ')) => {
            Some((1536, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'3'), Some(b'7'), Some(b' ')) => {
            Some((1537, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'3'), Some(b'8'), Some(b' ')) => {
            Some((1538, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'3'), Some(b'9'), Some(b' ')) => {
            Some((1539, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'4'), Some(b'0'), Some(b' ')) => {
            Some((1540, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'4'), Some(b'1'), Some(b' ')) => {
            Some((1541, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'4'), Some(b'2'), Some(b' ')) => {
            Some((1542, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'4'), Some(b'3'), Some(b' ')) => {
            Some((1543, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'4'), Some(b'4'), Some(b' ')) => {
            Some((1544, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'4'), Some(b'5'), Some(b' ')) => {
            Some((1545, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'4'), Some(b'6'), Some(b' ')) => {
            Some((1546, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'4'), Some(b'7'), Some(b' ')) => {
            Some((1547, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'4'), Some(b'8'), Some(b' ')) => {
            Some((1548, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'4'), Some(b'9'), Some(b' ')) => {
            Some((1549, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'5'), Some(b'0'), Some(b' ')) => {
            Some((1550, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'5'), Some(b'1'), Some(b' ')) => {
            Some((1551, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'5'), Some(b'2'), Some(b' ')) => {
            Some((1552, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'5'), Some(b'3'), Some(b' ')) => {
            Some((1553, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'5'), Some(b'4'), Some(b' ')) => {
            Some((1554, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'5'), Some(b'5'), Some(b' ')) => {
            Some((1555, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'5'), Some(b'6'), Some(b' ')) => {
            Some((1556, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'5'), Some(b'7'), Some(b' ')) => {
            Some((1557, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'5'), Some(b'8'), Some(b' ')) => {
            Some((1558, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'5'), Some(b'9'), Some(b' ')) => {
            Some((1559, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'6'), Some(b'0'), Some(b' ')) => {
            Some((1560, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'6'), Some(b'1'), Some(b' ')) => {
            Some((1561, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'6'), Some(b'2'), Some(b' ')) => {
            Some((1562, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'6'), Some(b'3'), Some(b' ')) => {
            Some((1563, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'6'), Some(b'4'), Some(b' ')) => {
            Some((1564, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'6'), Some(b'5'), Some(b' ')) => {
            Some((1565, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'6'), Some(b'6'), Some(b' ')) => {
            Some((1566, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'6'), Some(b'7'), Some(b' ')) => {
            Some((1567, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'6'), Some(b'8'), Some(b' ')) => {
            Some((1568, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'6'), Some(b'9'), Some(b' ')) => {
            Some((1569, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'7'), Some(b'0'), Some(b' ')) => {
            Some((1570, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'7'), Some(b'1'), Some(b' ')) => {
            Some((1571, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'7'), Some(b'2'), Some(b' ')) => {
            Some((1572, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'7'), Some(b'3'), Some(b' ')) => {
            Some((1573, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'7'), Some(b'4'), Some(b' ')) => {
            Some((1574, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'7'), Some(b'5'), Some(b' ')) => {
            Some((1575, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'7'), Some(b'6'), Some(b' ')) => {
            Some((1576, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'7'), Some(b'7'), Some(b' ')) => {
            Some((1577, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'7'), Some(b'8'), Some(b' ')) => {
            Some((1578, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'7'), Some(b'9'), Some(b' ')) => {
            Some((1579, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'8'), Some(b'0'), Some(b' ')) => {
            Some((1580, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'8'), Some(b'1'), Some(b' ')) => {
            Some((1581, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'8'), Some(b'2'), Some(b' ')) => {
            Some((1582, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'8'), Some(b'3'), Some(b' ')) => {
            Some((1583, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'8'), Some(b'4'), Some(b' ')) => {
            Some((1584, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'8'), Some(b'5'), Some(b' ')) => {
            Some((1585, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'8'), Some(b'6'), Some(b' ')) => {
            Some((1586, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'8'), Some(b'7'), Some(b' ')) => {
            Some((1587, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'8'), Some(b'8'), Some(b' ')) => {
            Some((1588, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'8'), Some(b'9'), Some(b' ')) => {
            Some((1589, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'9'), Some(b'0'), Some(b' ')) => {
            Some((1590, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'9'), Some(b'1'), Some(b' ')) => {
            Some((1591, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'9'), Some(b'2'), Some(b' ')) => {
            Some((1592, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'9'), Some(b'3'), Some(b' ')) => {
            Some((1593, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'9'), Some(b'4'), Some(b' ')) => {
            Some((1594, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'9'), Some(b'5'), Some(b' ')) => {
            Some((1595, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'9'), Some(b'6'), Some(b' ')) => {
            Some((1596, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'9'), Some(b'7'), Some(b' ')) => {
            Some((1597, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'9'), Some(b'8'), Some(b' ')) => {
            Some((1598, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'5'), Some(b'9'), Some(b'9'), Some(b' ')) => {
            Some((1599, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'0'), Some(b'0'), Some(b' ')) => {
            Some((1600, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'0'), Some(b'1'), Some(b' ')) => {
            Some((1601, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'0'), Some(b'2'), Some(b' ')) => {
            Some((1602, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'0'), Some(b'3'), Some(b' ')) => {
            Some((1603, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'0'), Some(b'4'), Some(b' ')) => {
            Some((1604, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'0'), Some(b'5'), Some(b' ')) => {
            Some((1605, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'0'), Some(b'6'), Some(b' ')) => {
            Some((1606, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'0'), Some(b'7'), Some(b' ')) => {
            Some((1607, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'0'), Some(b'8'), Some(b' ')) => {
            Some((1608, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'0'), Some(b'9'), Some(b' ')) => {
            Some((1609, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'1'), Some(b'0'), Some(b' ')) => {
            Some((1610, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'1'), Some(b'1'), Some(b' ')) => {
            Some((1611, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'1'), Some(b'2'), Some(b' ')) => {
            Some((1612, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'1'), Some(b'3'), Some(b' ')) => {
            Some((1613, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'1'), Some(b'4'), Some(b' ')) => {
            Some((1614, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'1'), Some(b'5'), Some(b' ')) => {
            Some((1615, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'1'), Some(b'6'), Some(b' ')) => {
            Some((1616, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'1'), Some(b'7'), Some(b' ')) => {
            Some((1617, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'1'), Some(b'8'), Some(b' ')) => {
            Some((1618, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'1'), Some(b'9'), Some(b' ')) => {
            Some((1619, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'2'), Some(b'0'), Some(b' ')) => {
            Some((1620, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'2'), Some(b'1'), Some(b' ')) => {
            Some((1621, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'2'), Some(b'2'), Some(b' ')) => {
            Some((1622, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'2'), Some(b'3'), Some(b' ')) => {
            Some((1623, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'2'), Some(b'4'), Some(b' ')) => {
            Some((1624, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'2'), Some(b'5'), Some(b' ')) => {
            Some((1625, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'2'), Some(b'6'), Some(b' ')) => {
            Some((1626, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'2'), Some(b'7'), Some(b' ')) => {
            Some((1627, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'2'), Some(b'8'), Some(b' ')) => {
            Some((1628, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'2'), Some(b'9'), Some(b' ')) => {
            Some((1629, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'3'), Some(b'0'), Some(b' ')) => {
            Some((1630, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'3'), Some(b'1'), Some(b' ')) => {
            Some((1631, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'3'), Some(b'2'), Some(b' ')) => {
            Some((1632, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'3'), Some(b'3'), Some(b' ')) => {
            Some((1633, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'3'), Some(b'4'), Some(b' ')) => {
            Some((1634, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'3'), Some(b'5'), Some(b' ')) => {
            Some((1635, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'3'), Some(b'6'), Some(b' ')) => {
            Some((1636, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'3'), Some(b'7'), Some(b' ')) => {
            Some((1637, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'3'), Some(b'8'), Some(b' ')) => {
            Some((1638, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'3'), Some(b'9'), Some(b' ')) => {
            Some((1639, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'4'), Some(b'0'), Some(b' ')) => {
            Some((1640, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'4'), Some(b'1'), Some(b' ')) => {
            Some((1641, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'4'), Some(b'2'), Some(b' ')) => {
            Some((1642, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'4'), Some(b'3'), Some(b' ')) => {
            Some((1643, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'4'), Some(b'4'), Some(b' ')) => {
            Some((1644, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'4'), Some(b'5'), Some(b' ')) => {
            Some((1645, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'4'), Some(b'6'), Some(b' ')) => {
            Some((1646, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'4'), Some(b'7'), Some(b' ')) => {
            Some((1647, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'4'), Some(b'8'), Some(b' ')) => {
            Some((1648, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'4'), Some(b'9'), Some(b' ')) => {
            Some((1649, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'5'), Some(b'0'), Some(b' ')) => {
            Some((1650, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'5'), Some(b'1'), Some(b' ')) => {
            Some((1651, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'5'), Some(b'2'), Some(b' ')) => {
            Some((1652, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'5'), Some(b'3'), Some(b' ')) => {
            Some((1653, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'5'), Some(b'4'), Some(b' ')) => {
            Some((1654, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'5'), Some(b'5'), Some(b' ')) => {
            Some((1655, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'5'), Some(b'6'), Some(b' ')) => {
            Some((1656, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'5'), Some(b'7'), Some(b' ')) => {
            Some((1657, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'5'), Some(b'8'), Some(b' ')) => {
            Some((1658, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'5'), Some(b'9'), Some(b' ')) => {
            Some((1659, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'6'), Some(b'0'), Some(b' ')) => {
            Some((1660, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'6'), Some(b'1'), Some(b' ')) => {
            Some((1661, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'6'), Some(b'2'), Some(b' ')) => {
            Some((1662, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'6'), Some(b'3'), Some(b' ')) => {
            Some((1663, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'6'), Some(b'4'), Some(b' ')) => {
            Some((1664, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'6'), Some(b'5'), Some(b' ')) => {
            Some((1665, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'6'), Some(b'6'), Some(b' ')) => {
            Some((1666, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'6'), Some(b'7'), Some(b' ')) => {
            Some((1667, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'6'), Some(b'8'), Some(b' ')) => {
            Some((1668, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'6'), Some(b'9'), Some(b' ')) => {
            Some((1669, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'7'), Some(b'0'), Some(b' ')) => {
            Some((1670, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'7'), Some(b'1'), Some(b' ')) => {
            Some((1671, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'7'), Some(b'2'), Some(b' ')) => {
            Some((1672, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'7'), Some(b'3'), Some(b' ')) => {
            Some((1673, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'7'), Some(b'4'), Some(b' ')) => {
            Some((1674, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'7'), Some(b'5'), Some(b' ')) => {
            Some((1675, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'7'), Some(b'6'), Some(b' ')) => {
            Some((1676, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'7'), Some(b'7'), Some(b' ')) => {
            Some((1677, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'7'), Some(b'8'), Some(b' ')) => {
            Some((1678, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'7'), Some(b'9'), Some(b' ')) => {
            Some((1679, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'8'), Some(b'0'), Some(b' ')) => {
            Some((1680, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'8'), Some(b'1'), Some(b' ')) => {
            Some((1681, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'8'), Some(b'2'), Some(b' ')) => {
            Some((1682, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'8'), Some(b'3'), Some(b' ')) => {
            Some((1683, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'8'), Some(b'4'), Some(b' ')) => {
            Some((1684, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'8'), Some(b'5'), Some(b' ')) => {
            Some((1685, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'8'), Some(b'6'), Some(b' ')) => {
            Some((1686, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'8'), Some(b'7'), Some(b' ')) => {
            Some((1687, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'8'), Some(b'8'), Some(b' ')) => {
            Some((1688, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'8'), Some(b'9'), Some(b' ')) => {
            Some((1689, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'9'), Some(b'0'), Some(b' ')) => {
            Some((1690, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'9'), Some(b'1'), Some(b' ')) => {
            Some((1691, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'9'), Some(b'2'), Some(b' ')) => {
            Some((1692, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'9'), Some(b'3'), Some(b' ')) => {
            Some((1693, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'9'), Some(b'4'), Some(b' ')) => {
            Some((1694, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'9'), Some(b'5'), Some(b' ')) => {
            Some((1695, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'9'), Some(b'6'), Some(b' ')) => {
            Some((1696, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'9'), Some(b'7'), Some(b' ')) => {
            Some((1697, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'9'), Some(b'8'), Some(b' ')) => {
            Some((1698, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'6'), Some(b'9'), Some(b'9'), Some(b' ')) => {
            Some((1699, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'0'), Some(b'0'), Some(b' ')) => {
            Some((1700, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'0'), Some(b'1'), Some(b' ')) => {
            Some((1701, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'0'), Some(b'2'), Some(b' ')) => {
            Some((1702, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'0'), Some(b'3'), Some(b' ')) => {
            Some((1703, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'0'), Some(b'4'), Some(b' ')) => {
            Some((1704, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'0'), Some(b'5'), Some(b' ')) => {
            Some((1705, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'0'), Some(b'6'), Some(b' ')) => {
            Some((1706, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'0'), Some(b'7'), Some(b' ')) => {
            Some((1707, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'0'), Some(b'8'), Some(b' ')) => {
            Some((1708, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'0'), Some(b'9'), Some(b' ')) => {
            Some((1709, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'1'), Some(b'0'), Some(b' ')) => {
            Some((1710, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'1'), Some(b'1'), Some(b' ')) => {
            Some((1711, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'1'), Some(b'2'), Some(b' ')) => {
            Some((1712, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'1'), Some(b'3'), Some(b' ')) => {
            Some((1713, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'1'), Some(b'4'), Some(b' ')) => {
            Some((1714, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'1'), Some(b'5'), Some(b' ')) => {
            Some((1715, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'1'), Some(b'6'), Some(b' ')) => {
            Some((1716, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'1'), Some(b'7'), Some(b' ')) => {
            Some((1717, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'1'), Some(b'8'), Some(b' ')) => {
            Some((1718, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'1'), Some(b'9'), Some(b' ')) => {
            Some((1719, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'2'), Some(b'0'), Some(b' ')) => {
            Some((1720, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'2'), Some(b'1'), Some(b' ')) => {
            Some((1721, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'2'), Some(b'2'), Some(b' ')) => {
            Some((1722, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'2'), Some(b'3'), Some(b' ')) => {
            Some((1723, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'2'), Some(b'4'), Some(b' ')) => {
            Some((1724, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'2'), Some(b'5'), Some(b' ')) => {
            Some((1725, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'2'), Some(b'6'), Some(b' ')) => {
            Some((1726, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'2'), Some(b'7'), Some(b' ')) => {
            Some((1727, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'2'), Some(b'8'), Some(b' ')) => {
            Some((1728, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'2'), Some(b'9'), Some(b' ')) => {
            Some((1729, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'3'), Some(b'0'), Some(b' ')) => {
            Some((1730, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'3'), Some(b'1'), Some(b' ')) => {
            Some((1731, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'3'), Some(b'2'), Some(b' ')) => {
            Some((1732, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'3'), Some(b'3'), Some(b' ')) => {
            Some((1733, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'3'), Some(b'4'), Some(b' ')) => {
            Some((1734, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'3'), Some(b'5'), Some(b' ')) => {
            Some((1735, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'3'), Some(b'6'), Some(b' ')) => {
            Some((1736, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'3'), Some(b'7'), Some(b' ')) => {
            Some((1737, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'3'), Some(b'8'), Some(b' ')) => {
            Some((1738, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'3'), Some(b'9'), Some(b' ')) => {
            Some((1739, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'4'), Some(b'0'), Some(b' ')) => {
            Some((1740, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'4'), Some(b'1'), Some(b' ')) => {
            Some((1741, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'4'), Some(b'2'), Some(b' ')) => {
            Some((1742, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'4'), Some(b'3'), Some(b' ')) => {
            Some((1743, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'4'), Some(b'4'), Some(b' ')) => {
            Some((1744, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'4'), Some(b'5'), Some(b' ')) => {
            Some((1745, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'4'), Some(b'6'), Some(b' ')) => {
            Some((1746, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'4'), Some(b'7'), Some(b' ')) => {
            Some((1747, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'4'), Some(b'8'), Some(b' ')) => {
            Some((1748, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'4'), Some(b'9'), Some(b' ')) => {
            Some((1749, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'5'), Some(b'0'), Some(b' ')) => {
            Some((1750, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'5'), Some(b'1'), Some(b' ')) => {
            Some((1751, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'5'), Some(b'2'), Some(b' ')) => {
            Some((1752, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'5'), Some(b'3'), Some(b' ')) => {
            Some((1753, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'5'), Some(b'4'), Some(b' ')) => {
            Some((1754, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'5'), Some(b'5'), Some(b' ')) => {
            Some((1755, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'5'), Some(b'6'), Some(b' ')) => {
            Some((1756, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'5'), Some(b'7'), Some(b' ')) => {
            Some((1757, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'5'), Some(b'8'), Some(b' ')) => {
            Some((1758, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'5'), Some(b'9'), Some(b' ')) => {
            Some((1759, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'6'), Some(b'0'), Some(b' ')) => {
            Some((1760, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'6'), Some(b'1'), Some(b' ')) => {
            Some((1761, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'6'), Some(b'2'), Some(b' ')) => {
            Some((1762, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'6'), Some(b'3'), Some(b' ')) => {
            Some((1763, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'6'), Some(b'4'), Some(b' ')) => {
            Some((1764, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'6'), Some(b'5'), Some(b' ')) => {
            Some((1765, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'6'), Some(b'6'), Some(b' ')) => {
            Some((1766, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'6'), Some(b'7'), Some(b' ')) => {
            Some((1767, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'6'), Some(b'8'), Some(b' ')) => {
            Some((1768, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'6'), Some(b'9'), Some(b' ')) => {
            Some((1769, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'7'), Some(b'0'), Some(b' ')) => {
            Some((1770, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'7'), Some(b'1'), Some(b' ')) => {
            Some((1771, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'7'), Some(b'2'), Some(b' ')) => {
            Some((1772, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'7'), Some(b'3'), Some(b' ')) => {
            Some((1773, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'7'), Some(b'4'), Some(b' ')) => {
            Some((1774, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'7'), Some(b'5'), Some(b' ')) => {
            Some((1775, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'7'), Some(b'6'), Some(b' ')) => {
            Some((1776, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'7'), Some(b'7'), Some(b' ')) => {
            Some((1777, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'7'), Some(b'8'), Some(b' ')) => {
            Some((1778, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'7'), Some(b'9'), Some(b' ')) => {
            Some((1779, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'8'), Some(b'0'), Some(b' ')) => {
            Some((1780, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'8'), Some(b'1'), Some(b' ')) => {
            Some((1781, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'8'), Some(b'2'), Some(b' ')) => {
            Some((1782, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'8'), Some(b'3'), Some(b' ')) => {
            Some((1783, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'8'), Some(b'4'), Some(b' ')) => {
            Some((1784, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'8'), Some(b'5'), Some(b' ')) => {
            Some((1785, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'8'), Some(b'6'), Some(b' ')) => {
            Some((1786, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'8'), Some(b'7'), Some(b' ')) => {
            Some((1787, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'8'), Some(b'8'), Some(b' ')) => {
            Some((1788, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'8'), Some(b'9'), Some(b' ')) => {
            Some((1789, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'9'), Some(b'0'), Some(b' ')) => {
            Some((1790, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'9'), Some(b'1'), Some(b' ')) => {
            Some((1791, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'9'), Some(b'2'), Some(b' ')) => {
            Some((1792, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'9'), Some(b'3'), Some(b' ')) => {
            Some((1793, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'9'), Some(b'4'), Some(b' ')) => {
            Some((1794, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'9'), Some(b'5'), Some(b' ')) => {
            Some((1795, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'9'), Some(b'6'), Some(b' ')) => {
            Some((1796, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'9'), Some(b'7'), Some(b' ')) => {
            Some((1797, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'9'), Some(b'8'), Some(b' ')) => {
            Some((1798, unsafe { buffer.get_unchecked(5..) }))
        }
        (Some(b'1'), Some(b'7'), Some(b'9'), Some(b'9'), Some(b' ')) => {
            Some((1799, unsafe { buffer.get_unchecked(5..) }))
        }
        _ => None,
    }
}

pub fn parse_color(buffer: &[u8]) -> Option<(u8, u8, u8)> {
    let buffer = buffer.get(0..6)?;
    let r: [u8; 2] = unsafe { buffer.get_unchecked(0..2).try_into().unwrap_unchecked() };
    let g: [u8; 2] = unsafe { buffer.get_unchecked(2..4).try_into().unwrap_unchecked() };
    let b: [u8; 2] = unsafe { buffer.get_unchecked(4..6).try_into().unwrap_unchecked() };

    #[inline]
    fn parse_hex(val: [u8; 2]) -> Option<u8> {
        match &val {
            b"00" => Some(0),
            b"01" => Some(1),
            b"02" => Some(2),
            b"03" => Some(3),
            b"04" => Some(4),
            b"05" => Some(5),
            b"06" => Some(6),
            b"07" => Some(7),
            b"08" => Some(8),
            b"09" => Some(9),
            b"0a" | b"0A" => Some(10),
            b"0b" | b"0B" => Some(11),
            b"0c" | b"0C" => Some(12),
            b"0d" | b"0D" => Some(13),
            b"0e" | b"0E" => Some(14),
            b"0f" | b"0F" => Some(15),
            b"10" => Some(16),
            b"11" => Some(17),
            b"12" => Some(18),
            b"13" => Some(19),
            b"14" => Some(20),
            b"15" => Some(21),
            b"16" => Some(22),
            b"17" => Some(23),
            b"18" => Some(24),
            b"19" => Some(25),
            b"1a" | b"1A" => Some(26),
            b"1b" | b"1B" => Some(27),
            b"1c" | b"1C" => Some(28),
            b"1d" | b"1D" => Some(29),
            b"1e" | b"1E" => Some(30),
            b"1f" | b"1F" => Some(31),
            b"20" => Some(32),
            b"21" => Some(33),
            b"22" => Some(34),
            b"23" => Some(35),
            b"24" => Some(36),
            b"25" => Some(37),
            b"26" => Some(38),
            b"27" => Some(39),
            b"28" => Some(40),
            b"29" => Some(41),
            b"2a" | b"2A" => Some(42),
            b"2b" | b"2B" => Some(43),
            b"2c" | b"2C" => Some(44),
            b"2d" | b"2D" => Some(45),
            b"2e" | b"2E" => Some(46),
            b"2f" | b"2F" => Some(47),
            b"30" => Some(48),
            b"31" => Some(49),
            b"32" => Some(50),
            b"33" => Some(51),
            b"34" => Some(52),
            b"35" => Some(53),
            b"36" => Some(54),
            b"37" => Some(55),
            b"38" => Some(56),
            b"39" => Some(57),
            b"3a" | b"3A" => Some(58),
            b"3b" | b"3B" => Some(59),
            b"3c" | b"3C" => Some(60),
            b"3d" | b"3D" => Some(61),
            b"3e" | b"3E" => Some(62),
            b"3f" | b"3F" => Some(63),
            b"40" => Some(64),
            b"41" => Some(65),
            b"42" => Some(66),
            b"43" => Some(67),
            b"44" => Some(68),
            b"45" => Some(69),
            b"46" => Some(70),
            b"47" => Some(71),
            b"48" => Some(72),
            b"49" => Some(73),
            b"4a" | b"4A" => Some(74),
            b"4b" | b"4B" => Some(75),
            b"4c" | b"4C" => Some(76),
            b"4d" | b"4D" => Some(77),
            b"4e" | b"4E" => Some(78),
            b"4f" | b"4F" => Some(79),
            b"50" => Some(80),
            b"51" => Some(81),
            b"52" => Some(82),
            b"53" => Some(83),
            b"54" => Some(84),
            b"55" => Some(85),
            b"56" => Some(86),
            b"57" => Some(87),
            b"58" => Some(88),
            b"59" => Some(89),
            b"5a" | b"5A" => Some(90),
            b"5b" | b"5B" => Some(91),
            b"5c" | b"5C" => Some(92),
            b"5d" | b"5D" => Some(93),
            b"5e" | b"5E" => Some(94),
            b"5f" | b"5F" => Some(95),
            b"60" => Some(96),
            b"61" => Some(97),
            b"62" => Some(98),
            b"63" => Some(99),
            b"64" => Some(100),
            b"65" => Some(101),
            b"66" => Some(102),
            b"67" => Some(103),
            b"68" => Some(104),
            b"69" => Some(105),
            b"6a" | b"6A" => Some(106),
            b"6b" | b"6B" => Some(107),
            b"6c" | b"6C" => Some(108),
            b"6d" | b"6D" => Some(109),
            b"6e" | b"6E" => Some(110),
            b"6f" | b"6F" => Some(111),
            b"70" => Some(112),
            b"71" => Some(113),
            b"72" => Some(114),
            b"73" => Some(115),
            b"74" => Some(116),
            b"75" => Some(117),
            b"76" => Some(118),
            b"77" => Some(119),
            b"78" => Some(120),
            b"79" => Some(121),
            b"7a" | b"7A" => Some(122),
            b"7b" | b"7B" => Some(123),
            b"7c" | b"7C" => Some(124),
            b"7d" | b"7D" => Some(125),
            b"7e" | b"7E" => Some(126),
            b"7f" | b"7F" => Some(127),
            b"80" => Some(128),
            b"81" => Some(129),
            b"82" => Some(130),
            b"83" => Some(131),
            b"84" => Some(132),
            b"85" => Some(133),
            b"86" => Some(134),
            b"87" => Some(135),
            b"88" => Some(136),
            b"89" => Some(137),
            b"8a" | b"8A" => Some(138),
            b"8b" | b"8B" => Some(139),
            b"8c" | b"8C" => Some(140),
            b"8d" | b"8D" => Some(141),
            b"8e" | b"8E" => Some(142),
            b"8f" | b"8F" => Some(143),
            b"90" => Some(144),
            b"91" => Some(145),
            b"92" => Some(146),
            b"93" => Some(147),
            b"94" => Some(148),
            b"95" => Some(149),
            b"96" => Some(150),
            b"97" => Some(151),
            b"98" => Some(152),
            b"99" => Some(153),
            b"9a" | b"9A" => Some(154),
            b"9b" | b"9B" => Some(155),
            b"9c" | b"9C" => Some(156),
            b"9d" | b"9D" => Some(157),
            b"9e" | b"9E" => Some(158),
            b"9f" | b"9F" => Some(159),
            b"a0" | b"A0" => Some(160),
            b"a1" | b"A1" => Some(161),
            b"a2" | b"A2" => Some(162),
            b"a3" | b"A3" => Some(163),
            b"a4" | b"A4" => Some(164),
            b"a5" | b"A5" => Some(165),
            b"a6" | b"A6" => Some(166),
            b"a7" | b"A7" => Some(167),
            b"a8" | b"A8" => Some(168),
            b"a9" | b"A9" => Some(169),
            b"aa" | b"AA" => Some(170),
            b"ab" | b"AB" => Some(171),
            b"ac" | b"AC" => Some(172),
            b"ad" | b"AD" => Some(173),
            b"ae" | b"AE" => Some(174),
            b"af" | b"AF" => Some(175),
            b"b0" | b"B0" => Some(176),
            b"b1" | b"B1" => Some(177),
            b"b2" | b"B2" => Some(178),
            b"b3" | b"B3" => Some(179),
            b"b4" | b"B4" => Some(180),
            b"b5" | b"B5" => Some(181),
            b"b6" | b"B6" => Some(182),
            b"b7" | b"B7" => Some(183),
            b"b8" | b"B8" => Some(184),
            b"b9" | b"B9" => Some(185),
            b"ba" | b"BA" => Some(186),
            b"bb" | b"BB" => Some(187),
            b"bc" | b"BC" => Some(188),
            b"bd" | b"BD" => Some(189),
            b"be" | b"BE" => Some(190),
            b"bf" | b"BF" => Some(191),
            b"c0" | b"C0" => Some(192),
            b"c1" | b"C1" => Some(193),
            b"c2" | b"C2" => Some(194),
            b"c3" | b"C3" => Some(195),
            b"c4" | b"C4" => Some(196),
            b"c5" | b"C5" => Some(197),
            b"c6" | b"C6" => Some(198),
            b"c7" | b"C7" => Some(199),
            b"c8" | b"C8" => Some(200),
            b"c9" | b"C9" => Some(201),
            b"ca" | b"CA" => Some(202),
            b"cb" | b"CB" => Some(203),
            b"cc" | b"CC" => Some(204),
            b"cd" | b"CD" => Some(205),
            b"ce" | b"CE" => Some(206),
            b"cf" | b"CF" => Some(207),
            b"d0" | b"D0" => Some(208),
            b"d1" | b"D1" => Some(209),
            b"d2" | b"D2" => Some(210),
            b"d3" | b"D3" => Some(211),
            b"d4" | b"D4" => Some(212),
            b"d5" | b"D5" => Some(213),
            b"d6" | b"D6" => Some(214),
            b"d7" | b"D7" => Some(215),
            b"d8" | b"D8" => Some(216),
            b"d9" | b"D9" => Some(217),
            b"da" | b"DA" => Some(218),
            b"db" | b"DB" => Some(219),
            b"dc" | b"DC" => Some(220),
            b"dd" | b"DD" => Some(221),
            b"de" | b"DE" => Some(222),
            b"df" | b"DF" => Some(223),
            b"e0" | b"E0" => Some(224),
            b"e1" | b"E1" => Some(225),
            b"e2" | b"E2" => Some(226),
            b"e3" | b"E3" => Some(227),
            b"e4" | b"E4" => Some(228),
            b"e5" | b"E5" => Some(229),
            b"e6" | b"E6" => Some(230),
            b"e7" | b"E7" => Some(231),
            b"e8" | b"E8" => Some(232),
            b"e9" | b"E9" => Some(233),
            b"ea" | b"EA" => Some(234),
            b"eb" | b"EB" => Some(235),
            b"ec" | b"EC" => Some(236),
            b"ed" | b"ED" => Some(237),
            b"ee" | b"EE" => Some(238),
            b"ef" | b"EF" => Some(239),
            b"f0" | b"F0" => Some(240),
            b"f1" | b"F1" => Some(241),
            b"f2" | b"F2" => Some(242),
            b"f3" | b"F3" => Some(243),
            b"f4" | b"F4" => Some(244),
            b"f5" | b"F5" => Some(245),
            b"f6" | b"F6" => Some(246),
            b"f7" | b"F7" => Some(247),
            b"f8" | b"F8" => Some(248),
            b"f9" | b"F9" => Some(249),
            b"fa" | b"FA" => Some(250),
            b"fb" | b"FB" => Some(251),
            b"fc" | b"FC" => Some(252),
            b"fd" | b"FD" => Some(253),
            b"fe" | b"FE" => Some(254),
            b"ff" | b"FF" => Some(255),
            _ => None,
        }
    }
    let r = parse_hex(r)?;
    let g = parse_hex(g)?;
    let b = parse_hex(b)?;
    Some((r, g, b))
}
