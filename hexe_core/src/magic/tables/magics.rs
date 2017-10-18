use magic::Magic;
use square::Square;

static MAGICS: [[Magic; 64]; 2] = [
    /* Bishop */ [Magic{num:0x2280828004C0020,mask:0x40201008040200,index:0x0,shift:58},Magic{num:0x301006019020C0,mask:0x402010080400,index:0x40,shift:59},Magic{num:0x808C0C0420400002,mask:0x4020100A00,index:0x60,shift:59},Magic{num:0x8104114208000000,mask:0x40221400,index:0x80,shift:59},Magic{num:0x74A021060030340,mask:0x2442800,index:0xA0,shift:59},Magic{num:0x4422011048000110,mask:0x204085000,index:0xC0,shift:59},Magic{num:0xC120210940200,mask:0x20408102000,index:0xE0,shift:59},Magic{num:0x6901440288031024,mask:0x2040810204000,index:0x100,shift:58},Magic{num:0x8432020122204C0,mask:0x20100804020000,index:0x140,shift:59},Magic{num:0x20401002020400A8,mask:0x40201008040000,index:0x160,shift:59},Magic{num:0x1009100102016090,mask:0x4020100A0000,index:0x180,shift:59},Magic{num:0x9014240400808020,mask:0x4022140000,index:0x1A0,shift:59},Magic{num:0xCC0422000000,mask:0x244280000,index:0x1C0,shift:59},Magic{num:0x4060262200010,mask:0x20408500000,index:0x1E0,shift:59},Magic{num:0x8C00820504024040,mask:0x2040810200000,index:0x200,shift:59},Magic{num:0x8010032101082000,mask:0x4081020400000,index:0x220,shift:59},Magic{num:0x14C0108C90420208,mask:0x10080402000200,index:0x240,shift:59},Magic{num:0x3000024A0A0420,mask:0x20100804000400,index:0x260,shift:59},Magic{num:0x184080802440208,mask:0x4020100A000A00,index:0x280,shift:57},Magic{num:0xB004800802004000,mask:0x402214001400,index:0x300,shift:57},Magic{num:0x4001000820180080,mask:0x24428002800,index:0x380,shift:57},Magic{num:0x488060A210100804,mask:0x2040850005000,index:0x400,shift:57},Magic{num:0x402120280846004,mask:0x4081020002000,index:0x480,shift:59},Magic{num:0xB32000901050180,mask:0x8102040004000,index:0x4A0,shift:59},Magic{num:0x90A05000050C0840,mask:0x8040200020400,index:0x4C0,shift:59},Magic{num:0x681040010100208,mask:0x10080400040800,index:0x4E0,shift:59},Magic{num:0x80308A2040101,mask:0x20100A000A1000,index:0x500,shift:57},Magic{num:0x204208004402C008,mask:0x40221400142200,index:0x580,shift:55},Magic{num:0x4001010000504000,mask:0x2442800284400,index:0x780,shift:55},Magic{num:0xC14A0091011120,mask:0x4085000500800,index:0x980,shift:57},Magic{num:0x4902022001451000,mask:0x8102000201000,index:0xA00,shift:59},Magic{num:0x2004002080828400,mask:0x10204000402000,index:0xA20,shift:59},Magic{num:0x202201000141010,mask:0x4020002040800,index:0xA40,shift:59},Magic{num:0x1033095040281003,mask:0x8040004081000,index:0xA60,shift:59},Magic{num:0xC020A0200030800,mask:0x100A000A102000,index:0xA80,shift:57},Magic{num:0xC004200800410050,mask:0x22140014224000,index:0xB00,shift:55},Magic{num:0x8020400185010,mask:0x44280028440200,index:0xD00,shift:55},Magic{num:0x53830A00010182,mask:0x8500050080400,index:0xF00,shift:57},Magic{num:0x1040100440102,mask:0x10200020100800,index:0xF80,shift:59},Magic{num:0x8C0100488180,mask:0x20400040201000,index:0xFA0,shift:59},Magic{num:0x4C130101000A4A6,mask:0x2000204081000,index:0xFC0,shift:59},Magic{num:0x104030446029090,mask:0x4000408102000,index:0xFE0,shift:59},Magic{num:0x2010103090048810,mask:0xA000A10204000,index:0x1000,shift:57},Magic{num:0x8000242039000800,mask:0x14001422400000,index:0x1080,shift:57},Magic{num:0x20904200600200,mask:0x28002844020000,index:0x1100,shift:57},Magic{num:0x412020049001200,mask:0x50005008040200,index:0x1180,shift:57},Magic{num:0x40830208A831200,mask:0x20002010080400,index:0x1200,shift:59},Magic{num:0x4316140404241090,mask:0x40004020100800,index:0x1220,shift:59},Magic{num:0x802020914400405,mask:0x20408102000,index:0x1240,shift:59},Magic{num:0x4502C250040080,mask:0x40810204000,index:0x1260,shift:59},Magic{num:0x8420010888900108,mask:0xA1020400000,index:0x1280,shift:59},Magic{num:0x9D28810184040040,mask:0x142240000000,index:0x12A0,shift:59},Magic{num:0x420C003102020010,mask:0x284402000000,index:0x12C0,shift:59},Magic{num:0x2008202002448800,mask:0x500804020000,index:0x12E0,shift:59},Magic{num:0x40501202015881,mask:0x201008040200,index:0x1300,shift:59},Magic{num:0x4008080804802204,mask:0x402010080400,index:0x1320,shift:59},Magic{num:0x1010804402208600,mask:0x2040810204000,index:0x1340,shift:58},Magic{num:0x2001904010C1200,mask:0x4081020400000,index:0x1380,shift:59},Magic{num:0x4810100044441030,mask:0xA102040000000,index:0x13A0,shift:59},Magic{num:0x10800008421200,mask:0x14224000000000,index:0x13C0,shift:59},Magic{num:0x8100000020204500,mask:0x28440200000000,index:0x13E0,shift:59},Magic{num:0x80331020091110,mask:0x50080402000000,index:0x1400,shift:59},Magic{num:0x4001A00409084500,mask:0x20100804020000,index:0x1420,shift:59},Magic{num:0xCA80809040050,mask:0x40201008040200,index:0x1440,shift:58},],
    /*  Rook  */ [Magic{num:0x880006090844004,mask:0x101010101017E,index:0x0,shift:52},Magic{num:0x40004020009000,mask:0x202020202027C,index:0x1000,shift:53},Magic{num:0x2080100018200080,mask:0x404040404047A,index:0x1800,shift:53},Magic{num:0x480100280080014,mask:0x8080808080876,index:0x2000,shift:53},Magic{num:0x160008900CA00600,mask:0x1010101010106E,index:0x2800,shift:53},Magic{num:0x80140001800200,mask:0x2020202020205E,index:0x3000,shift:53},Magic{num:0x8880008007000200,mask:0x4040404040403E,index:0x3800,shift:53},Magic{num:0x1000021000A4082,mask:0x8080808080807E,index:0x4000,shift:52},Magic{num:0x4800040002090,mask:0x1010101017E00,index:0x5000,shift:53},Magic{num:0x220401003200140,mask:0x2020202027C00,index:0x5800,shift:54},Magic{num:0x802000300088,mask:0x4040404047A00,index:0x5C00,shift:54},Magic{num:0x800A000843E03200,mask:0x8080808087600,index:0x6000,shift:54},Magic{num:0x4000808088000400,mask:0x10101010106E00,index:0x6400,shift:54},Magic{num:0x400800201802400,mask:0x20202020205E00,index:0x6800,shift:54},Magic{num:0x4142000200040118,mask:0x40404040403E00,index:0x6C00,shift:54},Magic{num:0x4800800880034300,mask:0x80808080807E00,index:0x7000,shift:53},Magic{num:0x200808000284003,mask:0x10101017E0100,index:0x7800,shift:53},Magic{num:0x1020021C21080,mask:0x20202027C0200,index:0x8000,shift:54},Magic{num:0x107010010200040,mask:0x40404047A0400,index:0x8400,shift:54},Magic{num:0x6000220040281200,mask:0x8080808760800,index:0x8800,shift:54},Magic{num:0x408004004014200,mask:0x101010106E1000,index:0x8C00,shift:54},Magic{num:0xC00808004002200,mask:0x202020205E2000,index:0x9000,shift:54},Magic{num:0x4000808005000200,mask:0x404040403E4000,index:0x9400,shift:54},Magic{num:0x200200004C0081,mask:0x808080807E8000,index:0x9800,shift:53},Magic{num:0x40813080024004,mask:0x101017E010100,index:0xA000,shift:53},Magic{num:0x4401080200080,mask:0x202027C020200,index:0xA800,shift:54},Magic{num:0x501110040A001,mask:0x404047A040400,index:0xAC00,shift:54},Magic{num:0x14286100100100,mask:0x8080876080800,index:0xB000,shift:54},Magic{num:0x8001040080080082,mask:0x1010106E101000,index:0xB400,shift:54},Magic{num:0x62008200040810,mask:0x2020205E202000,index:0xB800,shift:54},Magic{num:0x4059400500208,mask:0x4040403E404000,index:0xBC00,shift:54},Magic{num:0x82480048000C900,mask:0x8080807E808000,index:0xC000,shift:53},Magic{num:0x8008C00481800420,mask:0x1017E01010100,index:0xC800,shift:53},Magic{num:0x810400183002900,mask:0x2027C02020200,index:0xD000,shift:54},Magic{num:0x10002008801080,mask:0x4047A04040400,index:0xD400,shift:54},Magic{num:0x431004921001000,mask:0x8087608080800,index:0xD800,shift:54},Magic{num:0x2000452002008,mask:0x10106E10101000,index:0xDC00,shift:54},Magic{num:0x4000800201800400,mask:0x20205E20202000,index:0xE000,shift:54},Magic{num:0x2020304005830,mask:0x40403E40404000,index:0xE400,shift:54},Magic{num:0x8000C8800500,mask:0x80807E80808000,index:0xE800,shift:53},Magic{num:0x3080082000404000,mask:0x17E0101010100,index:0xF000,shift:53},Magic{num:0x1002C02010004000,mask:0x27C0202020200,index:0xF800,shift:54},Magic{num:0x800200102110041,mask:0x47A0404040400,index:0xFC00,shift:54},Magic{num:0x10209B0010021,mask:0x8760808080800,index:0x10000,shift:54},Magic{num:0x1009008010004,mask:0x106E1010101000,index:0x10400,shift:54},Magic{num:0x18020010B4020008,mask:0x205E2020202000,index:0x10800,shift:54},Magic{num:0x4020811440030,mask:0x403E4040404000,index:0x10C00,shift:54},Magic{num:0x40000040830E0004,mask:0x807E8080808000,index:0x11000,shift:53},Magic{num:0x240005024800080,mask:0x7E010101010100,index:0x11800,shift:53},Magic{num:0x250180C2002600,mask:0x7C020202020200,index:0x12000,shift:54},Magic{num:0x8000410490200100,mask:0x7A040404040400,index:0x12400,shift:54},Magic{num:0x800809004280080,mask:0x76080808080800,index:0x12800,shift:54},Magic{num:0x2000808400280280,mask:0x6E101010101000,index:0x12C00,shift:54},Magic{num:0x1040080060080,mask:0x5E202020202000,index:0x13000,shift:54},Magic{num:0xC02000108049200,mask:0x3E404040404000,index:0x13400,shift:54},Magic{num:0x200059028418C600,mask:0x7E808080808000,index:0x13800,shift:53},Magic{num:0x4000204300800011,mask:0x7E01010101010100,index:0x14000,shift:52},Magic{num:0x1000810200204292,mask:0x7C02020202020200,index:0x15000,shift:53},Magic{num:0x82429A002B04101,mask:0x7A04040404040400,index:0x15800,shift:53},Magic{num:0x8001000814100021,mask:0x7608080808080800,index:0x16000,shift:53},Magic{num:0x22001020080C02,mask:0x6E10101010101000,index:0x16800,shift:53},Magic{num:0x40200480904900A,mask:0x5E20202020202000,index:0x17000,shift:53},Magic{num:0x80990218029004,mask:0x3E40404040404000,index:0x17800,shift:53},Magic{num:0x440541008122,mask:0x7E80808080808000,index:0x18000,shift:52},],
];

const BISHOP_MAGICS_INDEX: usize = 0;
const ROOK_MAGICS_INDEX: usize = 1;

#[inline]
pub fn bishop_magic(square: Square) -> &'static Magic {
    &MAGICS[BISHOP_MAGICS_INDEX][square as usize]
}

#[inline]
pub fn rook_magic(square: Square) -> &'static Magic {
    &MAGICS[ROOK_MAGICS_INDEX][square as usize]
}