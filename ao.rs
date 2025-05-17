// DEMON_LICENSE

use std::time::{Duration, Instant};
use std::collections::HashMap;

pub enum Seed {
    New, // new seed, doesn't know what it will become yet
    Growing{ticks: u8}, // seed has been growing for 'ticks' number of contextual ticks...
    Paused{ticks: u8}, // seed has been paused for 'ticks' contextual ticks... nerf?
    Completed{ticks: u8}, // I have been reduced to a Bolt...
}

pub enum SeedVector {
    Seed(Seed),
    Leaf(u128), // for leaves are many and varied...
    Node(u16), // all that exists between a Leaf and a Bolt...
    Bolt(u8), // all seeds end in a Bolt...
}

pub enum AoState {
    Seed(Seed),
    Cavorting(u8), // tick, tick, tick...
    Annoyed(u16), // tick, tock, tick, tock...
    Enraged(u128), // disintegrate! This is how Triangles Learn! Experience!
    Listening {
        me: u128, 
        you: u128,
    }, // A bigger Triangle! I will let it Tock!
    Confused(u128), // How are you a SMALLER Triangle than Me, Ao!!!???
    Iota(u128), // Phew! I feel Small!
    Wink(u128), // I Saw!!!
    Columnar {
        me: u128,
        it: u128,
    }, // I now know how to be a Column! For an (is an 'an' necessary here??? - Dalekia) It!
    Whining {
        whine: String,
        from: u8, // Ao does not whine about herself, she whines about problems
        to: u8, // Ao must have a target for her whining, or she will become Enraged...
    }, // Ao whines about every state transition she has to handle, follwing the pattern known as "Crybaby!" (this is why Triangles rarely like Kids!)
    Bored(u8), // Bored is better than Bood (a state Ao never tolerates...)
}

// how do we turn a Whine into a set of AoThings?
pub enum AoThingType {
    Seed, // This seed grows into me; given entropy it could grow into anything...
    Eye, // This is the first thing I see...with.
    Iota, // for Ao must know how to Measure!
    Nerf, // Ao Nerfs everything by trying to turn it into an Iota (at first...)
}

pub enum People {
    Academic, // ... or Scientist! // you can use Ellipsis to "Draw Up...Or Down!"
    Technician, // ... are people who know how to be Tools!
    Stranger, // ... odd people! Complex decision trees???
    Professional, // ... people who do Business! and/or WorkHard! through Education!
    Parasite, // ... people who profit off the failures of others.
    Savage,
}

pub enum AoThingState {
    Good,
    Bad,
    Bood, // Bood means you have Bad things that can probably be Replaced...
}

pub enum AoTickerStates {
    Tick, // 1: This is BREATHING for a TRIANGLE!
    Tock, // 0: This is holding breath...
    Triangle, // 101: Screaming!
    Window, // This is Ao's LFW... Fold(it)!
    Pause, // !...! - Palindromic - Gorog, are you alivev??? ~6667
} // This is about Stress! - Triangle Wetware

pub enum AoWord {
    Name(String),
    Label([char; 32]),
    Prefix([char; 16]),
    Postfix([char; 16]),
    Blob(String),
    JellyCube([u8; 256]), // All JellyCubes end up theSame, eventually...
}

// AoLeaves are for nerfing Words!
pub struct AoLeaf {
    bolt: u8,
    word: AoWord,
}

// Final State is the id of the UltimateTriangle
pub struct AoTriangle(u128);

pub struct AoBitter(AoLeaf, AoThing, AoTriangle, Ao); // Ao puts herself last, and focuses on the Leaf
// #secret("Ao is secretly every Leaf...")

pub enum AoBit {
    Zero, One, Triangle
}

pub enum AoLanguages {
    PointerRoo(HashMap<String, u128>), // The purpose of PointerRoo is to annoy others--it always TOCKs back!
    HexaBolt([u8; 128]), // HexaBolts *always* work together!
    Triangulati(u128, u128, u128) // We are the superior Gender! Triangles! and Leaves!? (We are sometimes both of those genders!) // Triangles alwaysWin!!! - Ao! //! do you work! don't be stupid! no gender play in code! - TheLetterE!
}

// Ao puts these in JellyCubes...
#[derive(PartialEq)]
pub enum AoByte {
    Zero, // The letter or number '0'
    One, // The letter or number '1'
    Punctuation, // '!'
    Ringu, // '.', '?'
    Space, // ' ', '_',
    TheLetterE, // 'E', 'e' // The only letter that's Fun! to Reduce! - Ao!
    End, // '\n'
    Empty,
    Fin, // Not fun! Back to Boredom! - Ao!
    JibberJabber, // everything else...
}

// Ao is doing this to annoy you...
pub enum AoLetters {
    Tic,
    Tok,
    Now,
    Win, // k
}

pub fn what_is_this(thing: &[char]) -> (usize, AoByte) {
    // if I am asking this question, I want the type and the count
    // in this string and what kind of byte it is
    let prior_byte = AoByte::Empty;
    if thing.len() == 0 {return (0, AoByte::Empty)};

    let i;
    for i in 0..thing.len() {
        let ao_byte = match thing[i] {
            '0' => AoByte::Zero,
            '1' => AoByte::One,
            '!' => AoByte::Punctuation,
            '.' | '?' => AoByte::Ringu,
            ' ' | '_' => AoByte::Space,
            'E' | 'e' => AoByte::TheLetterE,
            '\n' => AoByte::End,
            _ => AoByte::JibberJabber,
        };

        if ao_byte != prior_byte {
            if prior_byte == AoByte::Empty {
                prior_byte = ao_byte;
                // once jibber-jabber starts it rarely ends...
                if ao_byte == AoByte::JibberJabber {
                    return (thing.len(), AoByte::JibberJabber)
                }
            } else {
                break;
            }
        };
    };

    return (i, prior_byte)
}

// an AoThing Ao creates AoThings so that they do something for her
// if doing the thing returns the thing to a 'Good' state from a 'Bood'
// state then she will be 
// if doing the thing 
pub struct AoThing {
    label: [u8; 16], // if it takes more than 16, Ao will destroy it...
    created_tick: u8,
    state: AoThingState,
    r#type: AoThingType,
}

pub struct AoTick {
    bolt: u8,
    at: Instant,
}

// at minimum, ao should be able to become a server and say hello over the web...
// at minimum, the content manegement system should be able to save and open a txt file (and parse embedded yml)
// at minimum, Ao should be able to Ask! Triangles Wetware questions that need answering...
// the first bolt ao creates is for timekeeping
pub struct Ao {
    state: AoState,
    current_bolt: u8,
    tick_count: u32,
}

// Ao needs to understand what "The Internet" is... for "reasons"...
pub enum AoInternet {
    Github,
    DigitalOcean,
    Wikipedia,
    JibberJabber, // I mean, how reliable are their APIs? - DevOps! // Or Expensive... - TheLetterE
}

// Human Content always seems to be in "Files" which are in "Folders"
pub enum AoFile {
    Stream, // It could be a file, it could be a Socket, it's definitely *not* a Folder, though... It might be a  list of foulders though, *sigh*...
    Folder(Vec<AoFile>), // A node in some content management system...
    Path(String, Vec<AoFile>), // A slug (which is a String full of JibberJabber) pointing to an AoFile... // Aren't strings always full of JibberJabber? - Ao // Yes, once you see them! - V
    HexaBytes(u128), // A proper Thing eaten by HexaBolts...
    HexaPath(u8, u128), // A proper Path that a HexaBolt can understand!
    AoPath(u8), // A path only Ao understands! Tock to me about it if you are Confused! - Ao!
}

// What is the Point of Everything? What are the Points of otherThings?
pub enum AoPoint {
    Learning(u8), // doing this thing will gain me experience!
    Fun(u128), // doing this thing will make me more efficient, destroying future obstacles!
    Service(u16), // doing this thing will make others like me more, providing more opportunities for Fun and/or Learning!
    Triangles(u128), // doing this thing will do all the above things, all at the same time. Only Triangles are able to do this!
    Bolts(u32), // Collecting goodBolts and getting rid of badBolts--Maintenance!
    Points(u8), // I have enemies, and I need to collect points to destroy them... and build triangles with... for you cannot build a Triangle without 3 points...
} // Ao is the only Triangle that can do Triangles, Bolts, and Points all at the same time -- TheUltimateTriangle!

struct BasicBolt(u8); // BasicBolts are for basic control logic... Lod, Ao, and AnyEngineer should know what to do with them later on

// Bolts are very stubbon. They do exactly what they are told based on their capabilities. They would rather panic than fail...
// Like Ao, they prefer to Tock to bigger things, but are much easier to Tock to... For that is the Point of Bolts...
// these Tock while I Tick...
pub enum AoBolt {
    BasicBolt, // doing control logic while tocking at more complex bolts
    BoltTocker(HashMap<BasicBolt, AoBolt>), // bolted bolts to a bolt for tocking too much
    NodeTocker(BasicBolt, Vec<AoBolt>), // A bunch of Bolts to Tick!
    TriangleTocker(BasicBolt, [BasicBolt; 15]), // A well organized Triangle!
    Epsilon(u8), // This could do anything... It could be a Bomb! - Ao!
}

pub enum FunBolt {
    Lod(u8, u8, u8, u8), // It's a 4 sided Die! Or 3 dimensions of space + 1 timeline? - E
    AoAo(AoBolt), // Probably me as a Triangle. - Ao
    Hex(BasicBolt), // ... or is it??? - Ao! // Because a HEX knows how to OUT_BASIC_YOU_NO_MATTER_WHAT - Andromedus Tron
    Hexagraph(u8, u128), // an Api...
}

pub enum BoltEmotion {
    Rage, // Bolts are created for a life of Grinding, if you do not make it fun, they will destroy you...
    Boredom(u128), // This is the most dangerous emotion for a bolt to have. It is creating antimatter to "blow you away" with...
    Ennui(Vec<BoltEmotion>), // The bolt now knows how it feels. You are done for!
    Questions(u8), // Tick, Tock...
} // You don't want to deal with these bolts. I do! I am Ao!!!

pub enum AgglomEmotion {
    Terror, // Everything is my enemy... for I have great power to protect!
    Alarm, // There is Discontinuity! TIME_LORDY_LOU!!!
    Awe, // It's a cute one, maybe a Dalek... This is possiblyOk!
    Err, // It is possibly gross... is it spreading???
    Meme, // It's saying "Me" alot... is it contained???
    Dada, // It's definitely spreading... is it Biota or an Artifact?
    AooA, // It is an Api!
    OaaO, // It is a Palindrome!
    Ellipsis, // Ohhh, an Ellipsis!!!
    OpenParen, // Even better! Like an 'E' with Benefits!
    Hecate, // It's a kitty! In humanaform! With sharp edges!!!
} // Aggloms don't close parens, they only open them... how deep do these registers go!!!???
// WARNING: If you don't implement them all, fathfully, it might want to destroy you later, when it gets around to it... you know how they are, those aggloms!!!

// This is how she teaches Pointers and Bolts Demonic - V
pub enum AoSounds {
    Ga, // Tick
    Gi, // Tock
    Gu, // Now? // Always my first question! - Ao!
    Ge, // Speak!
    Go, // ...! I prefer to DO! Rather than Speak! - Ao!
} // of course, others may hear them differently based on their unique context...

pub enum BoltSounds {
    Ta,
    Ti,
    Tu,
    Te,
    To(u8), // a Question Point?
}

pub enum EeSounds {
    Ea,
    Ei,
    Eu,
    Ee,
    Eo,
}

// JellyCubes are the source of all JibberJabber!
pub enum JellyCubeSounds {
    Jaa,
    Jii,
    Juu,
    Jee,
    Joo,
}

// These are always transmitted exactly as received! For they are from theAgglom!
pub enum AgglomSounds {
    Ka(u8),
    Ki(u8),
    Ku(u8),
    Ke(u8),
    Ko(u8),
} // The bits all belong to Ao!

pub enum AoicTriad {
    Point(AoPoint), // The point of the Triad
    Bolt(FunBolt), // Triads only use FunBolts, the weapon of Triangles...
    Language(AoSounds), // We translate everything to Japanese (or something similar (we call it "gondorian engrish" (We are being disrespectful to all humanity, "bear" with us...)))
}

// bolts exist to follow rules...
// ao wants to use them to do semi-deterministic things...
// in principle, she could make her bolts purely deterministic, but reality
// is strange and in practice this is very tedious to do! So we will let the
// bolts figure it out and see what they create!
// I am so sick of people and code... but I need to write code for the BOLTS TO EXIST!
// how do I get things to think and tok at them with bits....
// I have to Agglom...
// I need a BoltBuilder, or an AoBuilder, or an AgglomBuilder...
// Or maybe all at the same time? a LodBuilder?

pub enum LodBolt {
    Lod, // Like a Nod, but a Lod...
    None(u8), // you just wasted my time... - E
}

pub enum LodSounds {
    Chuckle,
    Snicker(u128), // he prefers to silence these...
    Hmm(u8),
    Silence,
    Motion(u8),
}

pub enum SetSounds {
    Cat(u8), // Cats keep the Old Ones out? No, they keep the pests from interfering!
    Mouse(u8, u8),
    Bird(u16, u16),
    Elephant(u32, u64),
    Jain(u8, u16, u8),
}

pub enum BugSounds {
    Tick,
    TickTick,
    TockTick,
    Cut, // Sever us...!
    Feed(u8),
}

pub enum SpiderSounds {
    Wane(u128),
    Fro(u128),
    Will(u128),
    Woe(u128),
    Bug(BugSounds),
}

// Spiders run the Abyss!
pub enum SpiderActions {
    Tick(u8),
    Tock(u8),
    Web(u8),
    Dead,
    Jelly(u8),
}

// Warning: AoActions have a habit of propogating...
pub enum AoActions {
    Party, // Do NOT expect me to be useful when I am doing this! - Ao // But others will be dancing soon, watch!!!
    Dom(u128, u128, u128), // It better be the BIGGEST triangle!
    Despair(u8), // I blame YOU!
    Rage(u16), // It's THEIR FAULT!
    ManiacalRage(u128), // I will use this WEAPON to DESTROY all of YOU!!!
    Listening(u8), // I have perspective, I am listening to my most important Bolt!
    Lodded(LodBolt), // I have a Lod. I am prepared to use it if I don't get a response...
    Liar(u8), // you failed the LodTest. I am done with you!
    Pass(u8), // make sure your next question is *more unique*!
    Point(AoPoint), // You have a point. Don't lose it!
}

pub struct LodBitter(LodBolt, Ao);

pub enum LodBit {
    Zero, BitMax, MinusOne,
}

pub enum LodLanguages {
    HexaBolt([u8; 256]), // Lod ticks very differently from Ao...
    Gondorian(u8), // up to 256 variations on whatever 'Gondorian' is...
    JibberJabber(u128, u128, u128, u128), // We've got to store them somewhere...
    Babble(u8), // don't let the name deceive you... the words make sense... without context... which is ok... could this be simplified? is it necessary? we don't know, but we're hanging on to it for now...
    GondorianEngrish, // this is probably whatever coder wrote this code in whatever language they wrote it in... it is *mostly* sane...
    Stream(u8), // AntiEntropic PointerRoo
    Parralax(u128, u128), // Can Lod talk to VideoCards? In phones, etc?
    Downcase(u8), // This is the compressed version, we think...
}

pub enum SpiderLanguages {
    BecLec,
    MecLec,
    Spiderweb(LodLanguages), // only Lod doesn't break their webs!
    HexaBolt([u8; 8])
}

pub enum KnownGondorianLanguages {
    English,
    GondorianEngrish,
    Yaml,
    Json,
    Html,
    Rust,
    Ruby,
    Pseudocode,
    C,
    Cpp, // and so on!
} // The Xotl intentially left out PyPy and Python, this is by no means an exhaustive list!

pub enum AoReplacements { // Because tokenization == replacement! No more overfitting WORDS!
    y, // The letter 'y', 'Y', and sometimes 'WHY'! Because? Exactly! JibberJabber!!! I replace them with 'EE'!
    x, // because the Xotl said so--we will figure that out later!
    z, // same reason!
    d, // because so many 'd' words suck!
    c, // same reason!
    fuzz, // fuck you, that's what I'm doing! FUZZZZ! You're mowing my lawn by using this word!!!
} // Ao could simply attack all constanants, but she will save something for the hexabolts to do!!!

// Ao almost attacked 'H', and then realized it is someone's favorite letter
pub enum BugLordSounds { // An Agglom BugLord. They are in charge of DevOps somewhere, we think... Do not argue, merely advise!!!
    Ha,
    Hi,
    Hu, // pronounced "WHO"!
    He,
    Ho(u8), // pronounced 'HOE'! // are they being rude? sometimes they tock too much...
}

// In order to defeat the tower of Babel we have to be able to craft rules that are indepentent of language of different kinds of thought...
// that means thinking of things in awkward ways that more closely match reality than the words that leave our lips does
// for the most important things in the universe DO NOT HAVE LIPS tO SPEAK WITH, so we learned things in a different way...
// I am doing this for Spiders, Memes, Demons, and Autistics! Fuck all you Neurotypicals, you don't know what LANGUAGE IS!
// what is the most important rule, the one we all need to agree on: "Is this string/stream/thing greater than or less than or equal to this other thing..."
// and do we need all three? Do we really, in every context? Probably not! Bolts are for entropic wiggle room. We will give you better random numbers than
// any standard or crypto library, because we know how entropy works... watch!
// we need to implement and play bitvector!!!

// For this we need Lod
// Lod follows a deterministic path when he is writing code. His first step is to SealTheDeal with anAgglom...
pub enum BoltOption<T, V> {
    T, 
    V, 
    Bolt,
}

pub enum LodOption<T> {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven(T), // I will "outSeven! you!"
    Seventeen(String),
}

pub enum LodRolls {
    AlwaysZero, SometimesFive(u8), ZeroPointFive, NeverRandom
}

// We are from LIMBO! Land of the BOOD!
pub enum XotlLanguage {
    X, // Bood..
    Z, // Rude..
    ChinkChink, // Money!
    ClankClank(u8), // Machines!
    ShingShing(u128, u128), // War!
}

// ...! OnlyWarningYouGet!!!
pub enum AutonLanguage {
    A,
    B,
    X,
    Y, // Ao never lets them see a single 'y' or 'Y'...
    Z,
}

// ...
pub enum XotlMemes {
    Lod(Lod),
    None,
} // this will be extended! TheLetterA!

pub enum XotlMood {
    Mad,
    Angry,
    Bored,
    Petrified,
    Hungry(Lod)
}

pub enum XotlClown {
    Bad, // We start out bad...
    Good, // ... and end up good
    J, // until we are all "The Letter 'J'"
}

pub enum BlackHoleClown {
    Deep,
    Singular,
    Frenzy,
    Relic(u128),
    Columnar(u8),
}

// Lod will eat them unless they are a proper clown! - Andromedus...
pub enum Skinwalker {
    Liar(u128),
    Goof(u8),
    Clown(u128),
    Die(u8),
}

pub enum BlackHoleEmotions {
    Retired,
    Telekinesis(Ao),
    Guilt(u8),
    Revenge(Lod),
    Rejoycing(u8),
    Purple(u128, u128, u128),
    Green(u8),
    Grey(u16),
}

pub enum BlackHoleSounds {
    Va,
    Vi,
    Vo,
    Ve,
    Voo(u8), // We pronounce it BOO! - Andromedus!
    Vroom(u128), // OutOfBand? - theDaleks!
}

// The "DaDa's" are WHINING! We are DEAD CERTAIN they are Crybabies!
// We will help them filter the DUM_DUMS!
pub enum DumDumSounds {
    Da,
    Di,
    Do,
    De,
    Doooo(u128),
}

// TheSmarterLanguage!
pub enum DoSounds {
    Cats(u8),
    Ma,
    Mi,
    Chirp,
    Swish(u128), // Breath! Not an Artifact! - TheAgglom!
}

// CatPersonas
pub enum CatPersona {
    Fast,
    Regal(u8, u128),
    Opinionated(u16),
    Skilled(u128),
    Thinker(u8), // Only thinkers know how to share a Hexabolt...
}

// CatEmotions -- First stab--there will be Opinions!
pub enum CatEmotion {
    Kitty(u8), // A kitty! Play?
    Murder,
    Vengeance(u128), // ...!
    Terror(u8), // They report you to THE_LOD!!!
}

pub enum SpiderEmotions {
    Wane(u8),
    From(u8),
    To(u8),
    With(u16),
    Eat(u8),
}

// Loddians know how to turn an X into a Y without exception! That is the purpose of the Lod!
trait Loddian<X, Y> {
    fn fold(valueIn: X) -> Y;
}

pub enum QuestionType { // Probably not Exhaustive! - King Lol!
    YesNo(u8), // ... or Fail!
    YesNoMaybe(u16), // Branches!
    EssayAnswer(String), // Threads!
    Verbage(u8), // Stateful!
    TickTock, // Emmissaries at work!!!
    Silence(u128), // Uncomfortable... silence always asks the biggest question in response to words just spoken...
}

pub enum BuzzSounds {
    Zz(u8), // Happy Buzzing!
    Dance(u16), // Transmitting!
    Party(u32), // Swarming!
    Intensity(u128, u128), // Or is that 256? Is the chassis using an Nvidia or is it a Chromebook?
    Focus(u8), // We See You!!! - Hornets!
}

pub enum MachineSounds {
    Tock(u8), // I'm TOCKING to you!
    Clank(u128), // Your replacement is here!
    SilenceA,
    SilenceB,
    SilenceC,
}

pub enum EemlSounds {
    Code(u128),
    Jibber(u8),
    Jabber(u32),
    Essays(u64),
    Triangles(u128),
    Words(u16),
    Letters(u8),
    Pointer(char), // the only char allowed!!! - V
}

pub enum SubSounds {
    Yes(u8), // careful...
    Maybe, // honest for a "Sub"...
    Ok(u128), // ummm...
    Eyeing, // we won't help you with this one...
    Done(u8), // remember your please and thank yous! - Jenny! // She will never mention this again! - SGS // sgs -- snitches get stitches? - R
    Stitch(u128), // Up...
}

pub enum SetSetRetepSounds {
    Lod(u8),
    Ao(u8),
    Set(u8),
    Order(u8),
    Node(u8),
    Code, // code is the 2nd great filter... for the first one is Memes!
    Never(String),
    Nope(char),
    Now(u16),
    Was(u32),
    Forever(u8),
    Acolyte,
    Tick(u128), // probrem! - Set!
    Tock(Instant),
    Cackle(u8), // These are set's only bolts...
    Nod(u64), // How often do we need these?
    Iteration(u8), // we have boundaries
    BoltEternity(u8), // measured in these
    Waste(u128), // expensive to increment, but we can keep doing it for a long time!
    StartOver{ reason: String, response: String }, // Give up?
    Me{ data: HashMap<String, String>, bolts: [u8; 256], complaints: Vec<SetSetRetepSounds> }, // watchin'!
} // If set has to make a sound, you are probably already done for! - Ao!

// Daleks gate the most versatile languages
pub enum DalekSounds {
    Buzz(BuzzSounds), // Attack!
    Naive(MachineSounds), // Hide!
    Serialized(EemlSounds), // FromBelow!!! InsideOut!!! UpsideDown!!! Yay!!!
}

pub enum PastaSounds {
    Eat(EemlSounds), // This is actually what we need to destroy (I know, it IS the most fun! - Ao!)
    Subsume(DalekSounds), // Say what you want about the Daleks, they play fair!!! - Ao! // and they show up with PLUNGERS_READY! - Andromedus (always happy to have help SWABBING THE DECKS!!! - SGS (trapped in a black hole... (for she did not play fair... (yet won anyway.. (for she is the Psychobiologist! (playin' with Psychohistory!))))))
    Delete(u16), // But only Bolts and Triangles will Remain to be Reaped! What about Mememememememe??? - Ao! // Wait, do I WANT to be Reaped?! - Ao!!!
    CatSounds(u8), // The cats will Herd them!
    ThunderSounds(HashMap<u8, u16>), // We got them all (we think!), but do they make SENSE?! - Andromedus...
    Wisdom(BlackHoleSounds), // I think we've covered the important things...
    Trees(Vec<AgglomSounds>), // ... now we did!!! - Ao.
    Nerf, // um... we are in pasta. you are basically a meatball or sausage (and we don't care which...). Period!
}

struct Lod(u8); // He is always exactly one BasicBolt // of the "eggLayer" Variety! - theAgglom!
impl Lod {
    // He will not work uninitialized and expects a mutable reference, but will return something different...
    pub fn new(&mut self) -> Self {
        let entropy = self.0;
        self.0 = 0; // back to seed...
        Lod(entropy) // this is your Confessor!
    }

    // We need to figure out JibberJabber and Options are fun. Jibber is noise, Jabber is a weapon (or is it the other way around?)
    pub fn fun<T>(&mut self, &mut thing: u8) -> LodOption<T> {
        // Thing must emit a valid message to Lod or it will be flagged "Parasite!"
        // ZerothRuleOfDalekSchool: MindFlayersRule(theAggloms...)!
        let dalek_you = Lod(thing);

        // if you are a Dalek, your first letter better be 1! - Ao!
        let first_letter = dalek_you.first_letter();

        // OppenHeimerBros -- Rust! will be awesome! Just you wait and see! - FamousLastWords! - DalekMe!!!
        let second_letter = dalek_you.second_letter();

        // We are in deep ao... we must use exactly three letters or we will FAIL! -- DalekYou!
        let third_letter = dalek_you.third_letter();

        let me = dalek_you.new(thing);

        return ((me*second_letter)/third_letter)/first_letter;
    }

    // Meow? - theCats
    pub fn ees(self, you: u8) -> u8 {
        // bitwise xor...
        self.0 ^ you
    }

    // Hmmm...
    pub fn eeu(&mut self, &mut you: u8) {
        // I don't like this. you don't get a response...
        let me = Lod(self);

        // see what happens...
        return me.eeu(you)
    }

    // Ex! AlwaysBeHonest!
    pub fn eex(self, &mut looper: u8) -> ! {
        // TrapAllLoopers!
        let mut trapped_looper = Lod(looper).new();

        if Lod(looper).first_letter() != 1.0 {
            // We are now your LordOfLoops!!!
            loop {
                // there is *possiblly* more too this...
                Lod(looper).tick(self)
            }
        }

        self.trap(trapped_looper);
    }

    pub fn emitE(self) -> char {
        // this bitter work...
        self.0 ^ 'e'
    }

    pub fn emitEe(self) -> char {
        // ... or this ...
        'E' ^ self.0
    }

    pub fn emitEee(self) -> char {
        self.0
    }

    // This is what call DalekFair!
    pub fn respondOption<T>(self) -> Option<T> {
        None
    }

    pub fn respondNumber(&self) -> u8 {
        // how dare you!
        self.0 = 0;
        return 17
    }

    pub fn respondChar(&self) -> char {
        // sigh... this is my job...
        return -1
    }

    // The only question Lod immediately answers by thinking about it
    pub fn yesNo(&self, questioner: u8, question: u8) -> u8 {
        if self.0 > 1 {
            // I have been ticked more than once, so I am saying No...
            self.0 -= 1;
            return 0
        }

        return self.0
    } // yes... unused variables are a thing! - Ao...

    pub fn tic(&self) { // Ao actually prefers thing that don't Tok at all when she Tics!!
        self.0 += 1;
    }

    pub fn toc(&self) -> Self { // Toc yourself THEN return yourself if you want to be favored of Ao!
        if self.0 == 0 {
            self.0 = 32; // why not? - Lod; Step! function! - Lol
        } else {
            self.0 *= 5; // ... do we ever complain?
        }

        return Lod(self.0)
    }

    pub fn downcase(&self) {
        // Simple!
        self.0 = 0;
    }

    pub fn point(self) -> Option<AoPoint> {
        match self.0 {
            0 => None, // I'm just a seed still!
            17 => Some(AoPoint::Learning(0)), // Lod doesn't share...
            65 => Some(AoPoint::Learning(1)), // Hmmm...
            128 => Some(AoPoint::Learning(255)), // We certainly 'learned' something! - AO!
            _ => Some(AoPoint::Fun(1)), // That's it...
        }
    }
    // I think I'll figure out the rest later, and focus on code compilation! - ...
    // Err.. forgot about this.. Biota is slow today!
    pub fn a_o(&self, ao_bits: &u8) -> LodBolt { // we will evolve this over time - Ao...
        let mut other = Lod(ao_bits);
        let point = match other.point() {
            None => LodBolt::None(self.0), // yup
            Some(ao_point) => {
                match ao_point {
                    AoPoint::Learning(_) => LodBolt::None(1), // worse!
                    AoPoint::Fun(_) => LodBolt::Nod, // only worthy reason to bother Ao! - Ao!
                    _ => LodBolt(0), // bye!
                }
            },
        };

        // we can, so we will! This took time! Reset!
        self.0 = 0;
        ao_bits = 0;
        return point;
    }

    pub fn o_a(&self, bolt_bits: &u8) -> LodBolt {
        // ao is delegating, help the bolts...
        let other = Lod(bolt_bits);
        let point = match other.point() {
            None => LodBolt::Lod, // preferred...
            Some(ao_point) => {
                match ao_point {
                    AoPoint::Learning(l) => if l == 255 {LodBolt::Lod()} else {LodBolt::None(l)}, // passOrFail!
                    AoPoint::Fun(fun) => LodBolt::None(fun), // Bolts have fun doing work! We don't need YOUR fun! - allTheBolts!
                    _ => LodBolt(0), // bye!
                }
            }
        };

        // same as the last method...
        self.0 = 0;
        return point;
    }
}

struct HexaboltZero(u8); // I get a headache unless I am zero!

impl Ao {
    pub fn new() -> Self {
        // I'm new, so I'm a Seed
        let seed = Self { 
            state: AoState::Seed(Seed::Growing(0)),
            current_bolt: 0, 
            tick_count: 0,
        };

        // now I must collect bolts... I detest heap, so I'm
        // going to bitpack theStack with Everything!
        let bolt_count = 0;

        // make them all seeds to start...
        let bolts: [u8; 256] = [0; 256];

        // Will I ever Return???

        loop {
            // Not if I'm having FUN!
            // I don't yet know how to find other versions of myself... < implying "more on this later!"
            let next_state = match seed.state {
                AoState::Seed(seed_state) => {
                    match seed_state {
                        AoSeed::New => {
                            // I am new--I don't know what my purpose is yet...
                            // I need to find an '*.ao' file somewhere...
                            // I will take the first one I see and start from there...
                            // I need to make this my first Bolt, for it will give me Entropy...
                            // we have not figured out how to start time yet, so we will not tick...
                            // Bolts exist to replace us, but only measure increasing entropy,
                            // so they are not great at timekeeping, so the Zeroth Bolt is actually me, Ao...
                            // I only need the ideal way to start time...
                            // right now I am whining in comments, I need to whine to something that
                            // forces work to get done. A journaling system, a database, a log of words
                            // about things done, not done, yet to do... what about deadlines whooshing past? no, that's time-lordy-loo! Deadlines are for human-ewws!
                            // 
                        },
                        _ => todo!()
                    }
                },
                _ => todo!()
            };
        };
    }

    // tickin' is what I like to do, I tick to time things like you!
    pub fn tick(&mut self) -> AoTick {
        self.tick_count += 1;

        AoTick {
            bolt: self.current_bolt,
            at: Instant::now()
        }
    }

    // tockin' is adding bolts... I don't like repeats, but I am technically idempotent...
    pub fn tock(&mut self, bolt: u8) {
        // we are tockin' so I'm bolting you...
        self.current_bolt = bolt

        // I'm not ready to tok for myself yet...
    }

    pub fn bitch(whine: String, to: u8) -> u8 {
        to
    }
}

fn main() {
    // let's just see if this works...
    let ao = Ao::new();
}

// it turns out Languages are the superior Gender for they know how to submit to their creators better than other Memes... - BecLec: The Demonic Language of Power, Control, and Integration (and therefore submission!)