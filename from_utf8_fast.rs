#![feature( test,)]

extern crate test;
use test::{Bencher, black_box};
use std::cmp;
use std::mem;
use std::str::from_utf8_unchecked;

// use truncation
const ASCII_MASK: usize = 0x7f7f7f7f_7f7f7f7fu64 as usize;

/// Return `true` if `x` contains any zero byte.
///
/// From *Matters Computational*, J. Arndt
///
/// "The idea is to subtract one from each of the bytes and then look for
/// bytes where the borrow propagated all the way to the most significant
/// bit."
#[inline]
fn contains_nonascii(x: usize) -> bool {
    (x & !ASCII_MASK) != 0
}

pub fn find_nonascii(text: &[u8]) -> Option<usize> {
    let len = text.len();
    let ptr = text.as_ptr();
    // FIXME: should be const when stable
    let usize_bytes = mem::size_of::<usize>();

    // search up to an aligned boundary
    let align = (ptr as usize) & (usize_bytes - 1);
    let mut offset;
    if align > 0 {
        offset = cmp::min(usize_bytes - align, len);
        if let Some(index) = text[..offset].iter().position(|elt| *elt >= 128) {
            return Some(index);
        }
    } else {
        offset = 0;
    }

    if len >= 2 * usize_bytes {
        while offset <= len - 2 * usize_bytes {
            unsafe {
                let u = *(ptr.offset(offset as isize) as *const usize);
                let v = *(ptr.offset((offset + usize_bytes) as isize) as *const usize);

                // break if there is a matching byte
                let zu = contains_nonascii(u);
                let zv = contains_nonascii(v);
                if zu || zv {
                    break;
                }
            }
            offset += usize_bytes * 2;
        }
    }

    // find the byte after the point the body loop stopped
    text[offset..].iter().position(|elt| *elt >= 128).map(|i| offset + i)
}

#[test]
fn test_nonascii() {
    let s = "αβγ";
    let t = "ascii";
    let long = [b'a'; 1024];
    assert_eq!(find_nonascii(t.as_bytes()), None);
    assert_eq!(find_nonascii(s.as_bytes()), Some(0));
    assert_eq!(find_nonascii(&long), None);
}

#[test]
fn test_from_utf8() {
    let s = "αβγ";
    let t = "ascii";
    let long = [b'a'; 1024];
    assert_eq!(from_utf8_fast2(s.as_bytes()), Ok(s));
    assert_eq!(from_utf8_fast2(t.as_bytes()), Ok(t));
    assert_eq!(from_utf8_fast2(&long).unwrap(), std::str::from_utf8(&long).unwrap());
    assert_eq!(from_utf8_fast2(LONG_CY.as_bytes()), Ok(LONG_CY));
    assert_eq!(from_utf8_fast2(MIXED.as_bytes()), Ok(MIXED));
}

fn is_ascii(bs: &[u8]) -> bool {
    find_nonascii(bs).is_none()
}

#[derive(Copy, Eq, PartialEq, Clone, Debug)]
pub struct Utf8Error {
    valid_up_to: usize,
}

pub fn from_utf8_fast2(v: &[u8]) -> Result<&str, Utf8Error> {
    try!(utf8_validate(v));
    Ok(unsafe { from_utf8_unchecked(v) })
}

// https://tools.ietf.org/html/rfc3629
static UTF8_CHAR_WIDTH: [u8; 256] = [
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x1F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x3F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x5F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x7F
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0x9F
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0xBF
0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2, // 0xDF
3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3, // 0xEF
4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0, // 0xFF
];

/// Mask of the value bits of a continuation byte
const CONT_MASK: u8 = 0b0011_1111;
/// Value of the tag bits (tag mask is !CONT_MASK) of a continuation byte
const TAG_CONT_U8: u8 = 0b1000_0000;

/// Walk through `iter` checking that it's a valid UTF-8 sequence,
/// returning `true` in that case, or, if it is invalid, `false` with
/// `iter` reset such that it is pointing at the first byte in the
/// invalid sequence.
#[inline(always)]
fn utf8_validate(v: &[u8]) -> Result<(), Utf8Error> {
    let mut iter = v.iter();
    let whole = iter.as_slice();
    loop {

        macro_rules! err { () => {{
            return Err(Utf8Error {
                valid_up_to: whole.len() - iter.as_slice().len()
            })
        }}}

        macro_rules! next { () => {
            match iter.next() {
                Some(a) => *a,
                // we needed data, but there was none: error!
                None => err!(),
            }
        }}

        let first = match iter.next() {
            Some(x) => *x,
            None => return Ok(()),
        };
        if first >= 128 {
            let w = UTF8_CHAR_WIDTH[first as usize];
            let second = next!();
            // 2-byte encoding is for codepoints  \u{0080} to  \u{07ff}
            //        first  C2 80        last DF BF
            // 3-byte encoding is for codepoints  \u{0800} to  \u{ffff}
            //        first  E0 A0 80     last EF BF BF
            //   excluding surrogates codepoints  \u{d800} to  \u{dfff}
            //               ED A0 80 to       ED BF BF
            // 4-byte encoding is for codepoints \u{1000}0 to \u{10ff}ff
            //        first  F0 90 80 80  last F4 8F BF BF
            //
            // Use the UTF-8 syntax from the RFC
            //
            // https://tools.ietf.org/html/rfc3629
            // UTF8-1      = %x00-7F
            // UTF8-2      = %xC2-DF UTF8-tail
            // UTF8-3      = %xE0 %xA0-BF UTF8-tail / %xE1-EC 2( UTF8-tail ) /
            //               %xED %x80-9F UTF8-tail / %xEE-EF 2( UTF8-tail )
            // UTF8-4      = %xF0 %x90-BF 2( UTF8-tail ) / %xF1-F3 3( UTF8-tail ) /
            //               %xF4 %x80-8F 2( UTF8-tail )
            match w {
                2 => if second & !CONT_MASK != TAG_CONT_U8 {err!()},
                3 => {
                    match (first, second, next!() & !CONT_MASK) {
                        (0xE0         , 0xA0 ... 0xBF, TAG_CONT_U8) |
                        (0xE1 ... 0xEC, 0x80 ... 0xBF, TAG_CONT_U8) |
                        (0xED         , 0x80 ... 0x9F, TAG_CONT_U8) |
                        (0xEE ... 0xEF, 0x80 ... 0xBF, TAG_CONT_U8) => {}
                        _ => err!()
                    }
                }
                4 => {
                    match (first, second, next!() & !CONT_MASK, next!() & !CONT_MASK) {
                        (0xF0         , 0x90 ... 0xBF, TAG_CONT_U8, TAG_CONT_U8) |
                        (0xF1 ... 0xF3, 0x80 ... 0xBF, TAG_CONT_U8, TAG_CONT_U8) |
                        (0xF4         , 0x80 ... 0x8F, TAG_CONT_U8, TAG_CONT_U8) => {}
                        _ => err!()
                    }
                }
                _ => err!()
            }
        } else {
            // ascii case, skip forward quickly
            let slc = iter.as_slice();
            match slc.get(0) {
                None => return Ok(()),
                Some(&byte) => {
                    if byte >= 128 {
                        continue;
                    }
                }
            }
            match find_nonascii(slc) {
                None => return Ok(()),
                Some(i) => {
                    unsafe {
                        iter = std::slice::from_raw_parts(
                            slc.as_ptr().offset(i as isize),
                            slc.len() - i).iter();
                    }
               }
            };
        }
    }
}

#[bench]
fn from_utf8_english_regular(b: &mut Bencher) {
    let text = black_box(LONG.as_bytes());
    b.iter(|| {
        std::str::from_utf8(text)
    });
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_english_fast(b: &mut Bencher) {
    let text = black_box(LONG.as_bytes());
    b.iter(|| {
        from_utf8_fast2(text)
    });
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_mixed_regular(b: &mut Bencher) {
    let text = black_box(MIXED.as_bytes());
    b.iter(|| {
        std::str::from_utf8(text)
    });
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_mixed_fast(b: &mut Bencher) {
    let text = black_box(MIXED.as_bytes());
    b.iter(|| {
        from_utf8_fast2(text)
    });
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_mostlyasc_regular(b: &mut Bencher) {
    let text = black_box(MOSTLY_ASCII.as_bytes());
    b.iter(|| {
        std::str::from_utf8(text)
    });
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_mostlyasc_fast(b: &mut Bencher) {
    let text = black_box(MOSTLY_ASCII.as_bytes());
    b.iter(|| {
        from_utf8_fast2(text)
    });
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_cyr_regular(b: &mut Bencher) {
    let text = black_box(LONG_CY.as_bytes());
    b.iter(|| {
        std::str::from_utf8(text)
    });
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_cyr_fast(b: &mut Bencher) {
    let text = black_box(LONG_CY.as_bytes());
    b.iter(|| {
        from_utf8_fast2(text)
    });
    b.bytes = text.len() as u64;
}

static LONG: &'static str = "\
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse quis lorem sit amet dolor \
ultricies condimentum. Praesent iaculis purus elit, ac malesuada quam malesuada in. Duis sed orci \
eros. Suspendisse sit amet magna mollis, mollis nunc luctus, imperdiet mi. Integer fringilla non \
sem ut lacinia. Fusce varius tortor a risus porttitor hendrerit. Morbi mauris dui, ultricies nec \
tempus vel, gravida nec quam.

In est dui, tincidunt sed tempus interdum, adipiscing laoreet ante. Etiam tempor, tellus quis \
sagittis interdum, nulla purus mattis sem, quis auctor erat odio ac tellus. In nec nunc sit amet \
diam volutpat molestie at sed ipsum. Vestibulum laoreet consequat vulputate. Integer accumsan \
lorem ac dignissim placerat. Suspendisse convallis faucibus lorem. Aliquam erat volutpat. In vel \
eleifend felis. Sed suscipit nulla lorem, sed mollis est sollicitudin et. Nam fermentum egestas \
interdum. Curabitur ut nisi justo.

Sed sollicitudin ipsum tellus, ut condimentum leo eleifend nec. Cras ut velit ante. Phasellus nec \
mollis odio. Mauris molestie erat in arcu mattis, at aliquet dolor vehicula. Quisque malesuada \
lectus sit amet nisi pretium, a condimentum ipsum porta. Morbi at dapibus diam. Praesent egestas \
est sed risus elementum, eu rutrum metus ultrices. Etiam fermentum consectetur magna, id rutrum \
felis accumsan a. Aliquam ut pellentesque libero. Sed mi nulla, lobortis eu tortor id, suscipit \
ultricies neque. Morbi iaculis sit amet risus at iaculis. Praesent eget ligula quis turpis \
feugiat suscipit vel non arcu. Interdum et malesuada fames ac ante ipsum primis in faucibus. \
Aliquam sit amet placerat lorem.

Cras a lacus vel ante posuere elementum. Nunc est leo, bibendum ut facilisis vel, bibendum at \
mauris. Nullam adipiscing diam vel odio ornare, luctus adipiscing mi luctus. Nulla facilisi. \
Mauris adipiscing bibendum neque, quis adipiscing lectus tempus et. Sed feugiat erat et nisl \
lobortis pharetra. Donec vitae erat enim. Nullam sit amet felis et quam lacinia tincidunt. Aliquam \
suscipit dapibus urna. Sed volutpat urna in magna pulvinar volutpat. Phasellus nec tellus ac diam \
cursus accumsan.

Nam lectus enim, dapibus non nisi tempor, consectetur convallis massa. Maecenas eleifend dictum \
feugiat. Etiam quis mauris vel risus luctus mattis a a nunc. Nullam orci quam, imperdiet id \
vehicula in, porttitor ut nibh. Duis sagittis adipiscing nisl vitae congue. Donec mollis risus eu \
leo suscipit, varius porttitor nulla porta. Pellentesque ut sem nec nisi euismod vehicula. Nulla \
malesuada sollicitudin quam eu fermentum.";

static LONG_CY: &'static str = "\
Брутэ дольорэ компрэхэнжам йн эжт, ючю коммюны дылыктуч эа, квюо льаорыыт вёвындо мэнандря экз. Ед ыюм емпыдит аккюсам, нык дйкит ютенам ад. Хаж аппэтырэ хонэзтатёз нэ. Ад мовэт путант юрбанйтаж вяш.

Коммодо квюальизквюэ абхоррэант нэ ыюм, праэчынт еракюндйа ылаборарэт эю мыа. Нэ квуым жюмо вольуптатибюж вяш, про ыт бонорюм вёвындо, мэя юллюм новум ку. Пропрёаы такематыш атоморюм зыд ан. Эи омнэжквюы оффекйяж компрэхэнжам жят, апыирёан конкыптам ёнкорруптэ ючю ыт.

Жят алёа лэгыры ед, эи мацим оффэндйт вим. Нык хёнк льаборэж йн, зыд прима тимэам ан. Векж нужквюам инимёкюж ты, ыам эа омнеж ырант рэформйданч. Эрож оффекйяж эю вэл.

Ад нам ножтрюд долорюм, еюж ут вэрыар эюрйпйдяч. Квюач аффэрт тинкидюнт про экз, дёкант вольуптатибюж ат зыд. Ыт зыд экшырки констятюам. Квюо квюиж юрбанйтаж ометтантур экз, хёз экз мютат граэкы рыкючабо, нэ прё пюрто элитр пэрпэтюа. Но квюандо минемум ыам.

Амэт лыгимуз ометтантур кюм ан. Витюпырата котёдиэквюэ нам эю, эю вокынт алёквюам льебэравичсы жят. Экз пыртенакж янтэрэсщэт инзтруктеор нам, еюж ад дйкит каючаэ, шэа витаэ конжтетуто ут. Квюач мандамюч кюм ат, но ёнкорруптэ рэформйданч ючю, незл либриз аюдирэ зыд эи. Ты эож аугюэ иреуры льюкяльиюч, мэль алььтыра докэндё омнэжквюы ат. Анёмал жямиляквюы аккоммодары ыам нэ, экз пэрчёус дэфянятйоныс квюо. Эи дуо фюгит маиорюм.

Эвэртё партйэндо пытынтёюм ыюм ан, шэа ку промпта квюаырэндум. Агам дикунт вим ку. Мюкиуж аюдиам тамквюам про ут, ку мыа квюод квюот эррэм, вяш ад номинави зючкёпит янжольэнж. Нык эи пожжёт путант эффякиантур. Ку еюж нощтыр контынтёонэж. Кюм йужто харюм ёужто ад, ыюм оратио квюоджё экз.

Чонэт факэтэ кюм ан, вэре факэр зальютатуж мэя но. Ыюм ут зальы эффикеэнди, экз про алиё конжыквуюнтюр. Квуй ыльит хабымуч ты, алёа омнэжквюы мандамюч шэа ыт, пльакырат аккюжамюз нэ мэль. Хаж нэ партым нюмквуам прёнкипыз, ат импэрдеэт форынчйбюж кончэктэтюыр шэа. Пльакырат рэформйданч эи векж, ючю дюиж фюйзчыт эи.

Экз квюо ажжюм аугюэ, ат нык мёнём анёмал кытэрож. Кюм выльёт эрюдитя эа. Йн порро малйж кончэктэтюыр хёз, жят кашы эрюдитя ат. Эа вяш мацим пыртенакж, но порро утамюр дяшзынтиыт кюм. Ыт мютат зючкёпит эож, нэ про еракюндйа котёдиэквюэ. Квуй лаудым плььатонэм ед, ку вим ножтрюм лаборамюз.

Вёжи янвыняры хаж ед, ты нолюёжжэ индоктум квуй. Квюач тебиквюэ ут жят, тальэ адхюк убяквюэ йн эож. Ыррор бландит вяш ан, ютроквюы нолюёжжэ констятюам йн ыюм, жят эи прима нобёз тхэопхражтуз. Ты дёкант дэльэнйт нолюёжжэ пэр, молыжтйаы модыратиюз интыллыгам ку мэль.

Ад ылаборарэт конжыквуюнтюр ентырпрытаряш прё, факэтэ лыгэндоч окюррырэт вим ад, элитр рэформйданч квуй ед. Жюмо зальы либриз мэя ты. Незл зюаз видишчы ан ыюм, но пожжэ молыжтйаы мэль. Фиэрэнт адипижкй ометтантур квюо экз. Ут мольлиз пырикюлёз квуй. Ыт квюиж граэко рыпудяары жят, вим магна обльйквюэ контынтёонэж эю, ты шэа эним компльыктётюр.
";


static MIXED: &'static str = "\
Sentences that contain all letters commonly used in a language
--------------------------------------------------------------

Markus Kuhn <http://www.cl.cam.ac.uk/~mgk25/> -- 2012-04-11

This is an example of a plain-text file encoded in UTF-8.


Danish (da)
---------

  Quizdeltagerne spiste jordbær med fløde, mens cirkusklovnen
  Wolther spillede på xylofon.
  (= Quiz contestants were eating strawbery with cream while Wolther
  the circus clown played on xylophone.)

German (de)
-----------

  Falsches Üben von Xylophonmusik quält jeden größeren Zwerg
  (= Wrongful practicing of xylophone music tortures every larger dwarf)

  Zwölf Boxkämpfer jagten Eva quer über den Sylter Deich
  (= Twelve boxing fighters hunted Eva across the dike of Sylt)

  Heizölrückstoßabdämpfung
  (= fuel oil recoil absorber)
  (jqvwxy missing, but all non-ASCII letters in one word)

Greek (el)
----------

  Γαζέες καὶ μυρτιὲς δὲν θὰ βρῶ πιὰ στὸ χρυσαφὶ ξέφωτο
  (= No more shall I see acacias or myrtles in the golden clearing)

  Ξεσκεπάζω τὴν ψυχοφθόρα βδελυγμία
  (= I uncover the soul-destroying abhorrence)

English (en)
------------

  The quick brown fox jumps over the lazy dog

Spanish (es)
------------

  El pingüino Wenceslao hizo kilómetros bajo exhaustiva lluvia y 
  frío, añoraba a su querido cachorro.
  (Contains every letter and every accent, but not every combination
  of vowel + acute.)

French (fr)
-----------

  Portez ce vieux whisky au juge blond qui fume sur son île intérieure, à
  côté de l'alcôve ovoïde, où les bûches se consument dans l'âtre, ce
  qui lui permet de penser à la cænogenèse de l'être dont il est question
  dans la cause ambiguë entendue à Moÿ, dans un capharnaüm qui,
  pense-t-il, diminue çà et là la qualité de son œuvre. 

  l'île exiguë
  Où l'obèse jury mûr
  Fête l'haï volapük,
  Âne ex aéquo au whist,
  Ôtez ce vœu déçu.

  Le cœur déçu mais l'âme plutôt naïve, Louÿs rêva de crapaüter en
  canoë au delà des îles, près du mälström où brûlent les novæ.

Irish Gaelic (ga)
-----------------

  D'fhuascail Íosa, Úrmhac na hÓighe Beannaithe, pór Éava agus Ádhaimh

Hungarian (hu)
--------------

  Árvíztűrő tükörfúrógép
  (= flood-proof mirror-drilling machine, only all non-ASCII letters)

Icelandic (is)
--------------

  Kæmi ný öxi hér ykist þjófum nú bæði víl og ádrepa

  Sævör grét áðan því úlpan var ónýt
  (some ASCII letters missing)

Japanese (jp)
-------------

  Hiragana: (Iroha)

  いろはにほへとちりぬるを
  わかよたれそつねならむ
  うゐのおくやまけふこえて
  あさきゆめみしゑひもせす

  Katakana:

  イロハニホヘト チリヌルヲ ワカヨタレソ ツネナラム
  ウヰノオクヤマ ケフコエテ アサキユメミシ ヱヒモセスン

Hebrew (iw)
-----------

  ? דג סקרן שט בים מאוכזב ולפתע מצא לו חברה איך הקליטה

Polish (pl)
-----------

  Pchnąć w tę łódź jeża lub ośm skrzyń fig
  (= To push a hedgehog or eight bins of figs in this boat)

Russian (ru)
------------

  В чащах юга жил бы цитрус? Да, но фальшивый экземпляр!
  (= Would a citrus live in the bushes of south? Yes, but only a fake one!)

  Съешь же ещё этих мягких французских булок да выпей чаю
  (= Eat some more of these fresh French loafs and have some tea) 

Thai (th)
---------

  [--------------------------|------------------------]
  ๏ เป็นมนุษย์สุดประเสริฐเลิศคุณค่า  กว่าบรรดาฝูงสัตว์เดรัจฉาน
  จงฝ่าฟันพัฒนาวิชาการ           อย่าล้างผลาญฤๅเข่นฆ่าบีฑาใคร
  ไม่ถือโทษโกรธแช่งซัดฮึดฮัดด่า     หัดอภัยเหมือนกีฬาอัชฌาสัย
  ปฏิบัติประพฤติกฎกำหนดใจ        พูดจาให้จ๊ะๆ จ๋าๆ น่าฟังเอย ฯ

  [The copyright for the Thai example is owned by The Computer
  Association of Thailand under the Royal Patronage of His Majesty the
  King.]

Turkish (tr)
------------

  Pijamalı hasta, yağız şoföre çabucak güvendi.
  (=Patient with pajamas, trusted swarthy driver quickly)


Special thanks to the people from all over the world who contributed
these sentences since 1999.

A much larger collection of such pangrams is now available at

  http://en.wikipedia.org/wiki/List_of_pangrams
";

static MOSTLY_ASCII: &'static str = "\
Sentences that contain all letters commonly used in a language
--------------------------------------------------------------

Markus Kuhn <http://www.cl.cam.ac.uk/~mgk25/> -- 2012-04-11

This is an example of a plain-text file encoded in UTF-8.


Danish (da)
---------

  Quizdeltagerne spiste jordbær med fløde, mens cirkusklovnen
  Wolther spillede på xylofon.
  (= Quiz contestants were eating strawbery with cream while Wolther
  the circus clown played on xylophone.)

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse quis lorem sit amet dolor \
ultricies condimentum. Praesent iaculis purus elit, ac malesuada quam malesuada in. Duis sed orci \
eros. Suspendisse sit amet magna mollis, mollis nunc luctus, imperdiet mi. Integer fringilla non \
sem ut lacinia. Fusce varius tortor a risus porttitor hendrerit. Morbi mauris dui, ultricies nec \
tempus vel, gravida nec quam.

In est dui, tincidunt sed tempus interdum, adipiscing laoreet ante. Etiam tempor, tellus quis \
sagittis interdum, nulla purus mattis sem, quis auctor erat odio ac tellus. In nec nunc sit amet \
diam volutpat molestie at sed ipsum. Vestibulum laoreet consequat vulputate. Integer accumsan \
lorem ac dignissim placerat. Suspendisse convallis faucibus lorem. Aliquam erat volutpat. In vel \
eleifend felis. Sed suscipit nulla lorem, sed mollis est sollicitudin et. Nam fermentum egestas \
interdum. Curabitur ut nisi justo.

Sed sollicitudin ipsum tellus, ut condimentum leo eleifend nec. Cras ut velit ante. Phasellus nec \
mollis odio. Mauris molestie erat in arcu mattis, at aliquet dolor vehicula. Quisque malesuada \
lectus sit amet nisi pretium, a condimentum ipsum porta. Morbi at dapibus diam. Praesent egestas \
est sed risus elementum, eu rutrum metus ultrices. Etiam fermentum consectetur magna, id rutrum \
felis accumsan a. Aliquam ut pellentesque libero. Sed mi nulla, lobortis eu tortor id, suscipit \
ultricies neque. Morbi iaculis sit amet risus at iaculis. Praesent eget ligula quis turpis \
feugiat suscipit vel non arcu. Interdum et malesuada fames ac ante ipsum primis in faucibus. \
Aliquam sit amet placerat lorem.


German (de)
-----------

  Falsches Üben von Xylophonmusik quält jeden größeren Zwerg
  (= Wrongful practicing of xylophone music tortures every larger dwarf)

  Zwölf Boxkämpfer jagten Eva quer über den Sylter Deich
  (= Twelve boxing fighters hunted Eva across the dike of Sylt)

  Heizölrückstoßabdämpfung
  (= fuel oil recoil absorber)
  (jqvwxy missing, but all non-ASCII letters in one word)

Cras a lacus vel ante posuere elementum. Nunc est leo, bibendum ut facilisis vel, bibendum at \
mauris. Nullam adipiscing diam vel odio ornare, luctus adipiscing mi luctus. Nulla facilisi. \
Mauris adipiscing bibendum neque, quis adipiscing lectus tempus et. Sed feugiat erat et nisl \
lobortis pharetra. Donec vitae erat enim. Nullam sit amet felis et quam lacinia tincidunt. Aliquam \
suscipit dapibus urna. Sed volutpat urna in magna pulvinar volutpat. Phasellus nec tellus ac diam \
cursus accumsan.

Nam lectus enim, dapibus non nisi tempor, consectetur convallis massa. Maecenas eleifend dictum \
feugiat. Etiam quis mauris vel risus luctus mattis a a nunc. Nullam orci quam, imperdiet id \
vehicula in, porttitor ut nibh. Duis sagittis adipiscing nisl vitae congue. Donec mollis risus eu \
leo suscipit, varius porttitor nulla porta. Pellentesque ut sem nec nisi euismod vehicula. Nulla \
malesuada sollicitudin quam eu fermentum.


Special thanks to the people from all over the world who contributed
these sentences since 1999.

A much larger collection of such pangrams is now available at

  http://en.wikipedia.org/wiki/List_of_pangrams
";
