#![feature(test)]
extern crate test;

mod current;
mod proposal;

use std::fs::File;
use std::io::Read;
use test::{black_box, Bencher};

#[test]
fn from_utf8_ascii() {
    let text = LONG.as_bytes();
    assert_eq!(
        current::run_utf8_validation(text, true).unwrap(),
        proposal::run_utf8_validation(text, true).unwrap()
    );
}

#[bench]
fn from_utf8_ascii_regular(b: &mut Bencher) {
    let text = black_box(LONG.as_bytes());
    b.iter(|| assert!(current::run_utf8_validation(text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_ascii_fast(b: &mut Bencher) {
    let text = black_box(LONG.as_bytes());
    b.iter(|| assert!(proposal::run_utf8_validation(text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[test]
fn from_utf8_mixed() {
    let text = MIXED.as_bytes();
    assert_eq!(
        current::run_utf8_validation(text, true).unwrap(),
        proposal::run_utf8_validation(text, true).unwrap()
    );
}

#[bench]
fn from_utf8_mixed_regular(b: &mut Bencher) {
    let text = black_box(MIXED.as_bytes());
    b.iter(|| assert!(current::run_utf8_validation(text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_mixed_fast(b: &mut Bencher) {
    let text = black_box(MIXED.as_bytes());
    b.iter(|| assert!(proposal::run_utf8_validation(text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[test]
fn from_utf8_mostlyasc() {
    let text = MOSTLY_ASCII.as_bytes();
    assert_eq!(
        current::run_utf8_validation(text, true).unwrap(),
        proposal::run_utf8_validation(text, true).unwrap()
    );
}

#[bench]
fn from_utf8_mostlyasc_regular(b: &mut Bencher) {
    let text = black_box(MOSTLY_ASCII.as_bytes());
    b.iter(|| assert!(current::run_utf8_validation(text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_mostlyasc_fast(b: &mut Bencher) {
    let text = black_box(MOSTLY_ASCII.as_bytes());
    b.iter(|| assert!(proposal::run_utf8_validation(text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[test]
fn from_utf8_cyr() {
    let text = LONG_CY.as_bytes();
    assert_eq!(
        current::run_utf8_validation(text, true).unwrap(),
        proposal::run_utf8_validation(text, true).unwrap()
    );
}

#[bench]
fn from_utf8_cyr_regular(b: &mut Bencher) {
    let text = black_box(LONG_CY.as_bytes());
    b.iter(|| assert!(current::run_utf8_validation(text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_cyr_fast(b: &mut Bencher) {
    let text = black_box(LONG_CY.as_bytes());
    b.iter(|| assert!(proposal::run_utf8_validation(text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[test]
fn from_utf8_enwik8() {
    let mut text = Vec::new();
    let mut f = File::open("enwik8").unwrap();
    f.read_to_end(&mut text).unwrap();
    assert_eq!(
        current::run_utf8_validation(&text, true).unwrap(),
        proposal::run_utf8_validation(&text, true).unwrap()
    );
}

#[bench]
fn from_utf8_enwik8_regular(b: &mut Bencher) {
    let mut text = Vec::new();
    let mut f = File::open("enwik8").unwrap();
    f.read_to_end(&mut text).unwrap();
    b.iter(|| assert!(current::run_utf8_validation(&text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_enwik8_fast(b: &mut Bencher) {
    let mut text = Vec::new();
    let mut f = File::open("enwik8").unwrap();
    f.read_to_end(&mut text).unwrap();
    b.iter(|| assert!(proposal::run_utf8_validation(&text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[test]
fn from_utf8_jawik10() {
    let mut text = Vec::new();
    let mut f = File::open("jawik10").unwrap();
    f.read_to_end(&mut text).unwrap();
    assert_eq!(
        current::run_utf8_validation(&text, true).unwrap(),
        proposal::run_utf8_validation(&text, true).unwrap()
    );
}

#[bench]
fn from_utf8_jawik10_regular(b: &mut Bencher) {
    let mut text = Vec::new();
    let mut f = File::open("jawik10").unwrap();
    f.read_to_end(&mut text).unwrap();
    b.iter(|| assert!(current::run_utf8_validation(&text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_jawik10_fast(b: &mut Bencher) {
    let mut text = Vec::new();
    let mut f = File::open("jawik10").unwrap();
    f.read_to_end(&mut text).unwrap();
    b.iter(|| assert!(proposal::run_utf8_validation(&text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[test]
fn from_utf8_2_bytes() {
    let s = String::from("£").repeat(200);
    let text = s.as_bytes();
    assert_eq!(
        current::run_utf8_validation(&text, true).unwrap(),
        proposal::run_utf8_validation(&text, true).unwrap()
    );
}

#[bench]
fn from_utf8_2_bytes_regular(b: &mut Bencher) {
    let s = String::from("£").repeat(200);
    let text = black_box(s.as_bytes());
    b.iter(|| assert!(current::run_utf8_validation(&text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_2_bytes_fast(b: &mut Bencher) {
    let s = String::from("£").repeat(200);
    let text = black_box(s.as_bytes());
    b.iter(|| assert!(proposal::run_utf8_validation(&text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[test]
fn from_utf8_3_bytes() {
    let s = String::from("ࠀက").repeat(200);
    let text = s.as_bytes();
    assert_eq!(
        current::run_utf8_validation(&text, true).unwrap(),
        proposal::run_utf8_validation(&text, true).unwrap()
    );
}

#[bench]
fn from_utf8_3_bytes_regular(b: &mut Bencher) {
    let s = String::from("ࠀက").repeat(200);
    let text = black_box(s.as_bytes());
    b.iter(|| assert!(current::run_utf8_validation(&text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_3_bytes_fast(b: &mut Bencher) {
    let s = String::from("ࠀက").repeat(200);
    let text = black_box(s.as_bytes());
    b.iter(|| assert!(proposal::run_utf8_validation(&text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[test]
fn from_utf8_4_bytes() {
    let s = String::from("𐀀񀀀􀀀").repeat(200);
    let text = s.as_bytes();
    assert_eq!(
        current::run_utf8_validation(&text, true).unwrap(),
        proposal::run_utf8_validation(&text, true).unwrap()
    );
}

#[bench]
fn from_utf8_4_bytes_regular(b: &mut Bencher) {
    let s = String::from("𐀀񀀀􀀀").repeat(200);
    let text = black_box(s.as_bytes());
    b.iter(|| assert!(current::run_utf8_validation(&text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_4_bytes_fast(b: &mut Bencher) {
    let s = String::from("𐀀񀀀􀀀").repeat(200);
    let text = black_box(s.as_bytes());
    b.iter(|| assert!(proposal::run_utf8_validation(&text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[test]
fn from_utf8_all_bytes() {
    let s = String::from("£ࠀက𐀀񀀀􀀀").repeat(200);
    let text = s.as_bytes();
    assert_eq!(
        current::run_utf8_validation(&text, true).unwrap(),
        proposal::run_utf8_validation(&text, true).unwrap()
    );
}

#[bench]
fn from_utf8_all_bytes_regular(b: &mut Bencher) {
    let s = String::from("£ࠀက𐀀񀀀􀀀").repeat(200);
    let text = black_box(s.as_bytes());
    b.iter(|| assert!(current::run_utf8_validation(&text, false).is_ok()));
    b.bytes = text.len() as u64;
}

#[bench]
fn from_utf8_all_bytes_fast(b: &mut Bencher) {
    let s = String::from("£ࠀက𐀀񀀀􀀀").repeat(200);
    let text = black_box(s.as_bytes());
    b.iter(|| assert!(proposal::run_utf8_validation(&text, false).is_ok()));
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
