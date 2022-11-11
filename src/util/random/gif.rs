use rand::seq::SliceRandom;

const GOOD_NIGHT_GIF_URLS: [&str; 5] = [
    "https://media.tenor.com/hopHX-in69QAAAAM/anime-girl.gif",
    "https://media.tenor.com/Jqg5OP3rPqwAAAAM/anime-sleep.gif",
    "https://media.tenor.com/CLKPAdlPCwUAAAAM/sad-cute.gif",
    "https://media.tenor.com/IFueawYnBokAAAAM/hail-all.gif",
    "https://media.tenor.com/FdgeECXUQZYAAAAM/goodnight-eveyrone.gif"
];

pub fn random_gn() -> &'static str {
    let mut rng = rand::thread_rng();

    GOOD_NIGHT_GIF_URLS
        .choose(&mut rng)
        .expect("GOOD_NIGHT_GIF_URLS was empty")
}

const GOOD_MORNING_GIF_URLS: [&str; 5] = [
    "https://media.tenor.com/oXWg44r_GOkAAAAM/good-morning-anime-girl-good-morning.gif",
    "https://media.tenor.com/DbSevHAsGHQAAAAM/shinoa-good-morning.gif",
    "https://media.tenor.com/GFdmzb8gVssAAAAM/good-morning-morning.gif",
    "https://media.tenor.com/fFztGkHmg_0AAAAM/araragi-karen.gif",
    "https://media.tenor.com/siyjThcv6uwAAAAM/little-witch.gif"
];

pub fn random_gm() -> &'static str {
    let mut rng = rand::thread_rng();

    GOOD_MORNING_GIF_URLS
        .choose(&mut rng)
        .expect("GOOD_MORNING_GIF_URLS was empty")
}

const NAP_GIF_URLS: [&str; 5] = [
    "https://media.tenor.com/HQMew39r_-EAAAAM/go-to-sleep-sleep.gif",
    "https://media.tenor.com/QInRffsEJyMAAAAM/tacam-ltg.gif",
    "https://media.tenor.com/HlSPF1Ls3b0AAAAM/go-to-sleep.gif",
    "https://media.tenor.com/erO3QRPa69UAAAAM/low-tier-god-ltg.gif",
    "https://media.tenor.com/0cwkXKvn9Q8AAAAM/shikinami-cheerleader.gif"
];

pub fn random_nap() -> &'static str {
    let mut rng = rand::thread_rng();

    NAP_GIF_URLS
        .choose(&mut rng)
        .expect("NAP_GIF_URLS was empty")
}

const WAKEY_GIF_URLS: [&str; 5] = [
    "https://media.tenor.com/j9dLwzX4AtwAAAAM/anime-date-a-live.gif",
    "https://media.tenor.com/80ZNJEEnjrYAAAAM/anime-poke.gif",
    "https://media.tenor.com/LSbc6GpcibQAAAAM/azumanga-daioh-azumanga.gif",
    "https://media.tenor.com/RFhN3vT8ClUAAAAM/dog-doge.gif",
    "https://media.tenor.com/obRYotBbTkUAAAAM/wakeup-good-morning.gif"
];

pub fn random_wakey() -> &'static str {
    let mut rng = rand::thread_rng();

    WAKEY_GIF_URLS
        .choose(&mut rng)
        .expect("WAKEY_GIF_URLS was empty")
}