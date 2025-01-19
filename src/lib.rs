pub const SECONDS_IN_A_DAY: i64 = 60 * 60 * 24;
pub const SECONDS_IN_TWO_DAYS: i64 = 2 * SECONDS_IN_A_DAY;

pub const D1Y: i32 = 2025;
pub const D1M: u32 = 1;
pub const D1D: u32 = 22;
pub const D1H_UTC: u32 = 5;
pub const D1M_UTC: u32 = 0;
pub const D1S_UTC: u32 = 0;

pub const BEFORE_INAUGURATION_TEMPLATES: &'static [&'static str]  = &[
    "If Donald Trump can keep his word, Ross Ulbricht will be free in less than <DAYS> days, <HOURS> hours, and <MINUTES> minutes\n\n#freeross",
    "DAY ONE: T-minus <DAYS> days, <HOURS> hours, and <MINUTES> minutes. \n\n#freeross",
    "Bring Ross home! <DAYS> days, <HOURS> hours, and <MINUTES> minutes.\n\n#freeross",
    "Stay humble. Stack sats. Free Ross.\n\n<DAYS> days, <HOURS> hours, <MINUTES> minutes.\n\n#freeross",
    "Is it DAY ONE or \"one day...\"? We will see in less than <DAYS> days, <HOURS> hours, and <MINUTES> minutes.\n\n#freeross",
    "\"If you vote for me, on day one, I will commute the sentence of Ross Ulbricht.\" --Donald J Trump:\n\nWill he keep his word?\n\nWe will know less than <DAYS> days, <HOURS> hours, and <MINUTES> minutes.\n\n#freeross"
];

pub const DAY_OF_INAUGURATION_TEMPLATES: &'static [&'static str] = &[
    "TODAY IS YOUR DAY DONNIE. You have <DAYS> days, <HOURS> hours, and <SECONDS> seconds to make good on your campaign promise to FREE ROSS DAY ONE",
    "THE SUSPENSE. Donald Trump has <DAYS> days, <HOURS> hours, and <MINUTES> minutes to FREE ROSS DAY ONE.",
    "Congratulations, President Trump. The world is watching. Do the right thing, and free Ross! You have <DAYS> days, <HOURS> hours, and <MINUTES> minutes to make good on your promise!"
];

pub const AFTER_INAUGURATION_NEGATIVE_TEMPLATES: &'static [&'static str] = &[
    "TODAY IS THE DAY DONALD TRUMP. Ross should have been free <DAYS> days, <HOURS> hours, and <MINUTES> minutes ago. DO THE RIGHT THING.",
    "Donald Trump said he would free Ross on \"Day One\". Day One was <DAYS> days, <HOURS> hours, and <MINUTES> minutes ago.",
    "Getting real sick of your shit, Donno. The deadline was <DAYS> days, <HOURS> hours, and <MINUTES> minutes ago.",
    "Donald, I am just a cron job. I can't do much but diff some dates. \n\n But you, sir, can free Ross, like you were promised you would do <DAYS> days, <HOURS> hours, and <MINUTES> minutes ago.",
];

pub const ROSS_IS_FREE: &'static [&'static str] = &[
    "HE DID IT. DON, YOU ABSOLUTE MAD LAD. Ross is free! Congratulations Ross! Congratulations Lyn!",
    "ROSS IS FREE. Justice served!",
    "There is a higher court than courts of justice and that is the court of conscience. It supercedes all other courts. --Mahatma Ghandi",
    "Law and order exist for the purpose of establishing justice and when they fail in this purpose they become the dangerously structured dams that block the flow of social progress. --Martin Luther King Jr.",
    "I do not pretend to understand the moral universe, the arc is a long one, my eye reaches but little ways.\n\nI cannot calculate the curve and complete the figure by the experience of sight; I can divine it by conscience.\n\nBut from what I see I am sure it bends towards justice. --Theodore Parker"
];

pub const RELEVANT_FREEROSS_NOTES: &'static [&'static str] = &[
    "nostr:nevent1qqsqp2w8azjzx5lrfvsayeaksjw9a9fzzzl6zrp497k5vlhprgnut5cpzdmhxue69uhhwmm59e6hg7r09ehkuef0qgsxua0hju3e0j3jjhs0fjs0h3htnnreh6zm4lw4d0fhsgsv4rhwwnsrqsqqqqqpty3szm",
    "nostr:nevent1qqs2uf25kg8hcys5tsxdtrepzmd89zn9t0vamwu7v77mfhuxd39z5kcpz4mhxue69uhhyetvv9ujuerpd46hxtnfduhsygpl5r0fqr63tlcl92wz89ada09zt4tu7kw9lc62mmzpz3hprm6rqcpsgqqqqqqshahznr",
    "nostr:nevent1qqsgwhw288rftz85sukvf6rqks2qzt5km7x8pzcsk6h9m3mgeq8mqvspzemhxue69uhkummnw3ex2mrfw3jhxtn0wfnj7qgkwaehxw309ajkgetw9ehx7um5wghxcctwvshszrnhwden5te0dehhxtnvdakz7qg4waehxw309aex2mrp0yhxgctdw4eju6t09uq3jamnwvaz7tmjv4kxz7fwwdhx7un59eek7cmfv9kz7q3ql49ccxu5kvpccapwq269yjnc9jj5tteunfgkhl774hn2v0sdzq0sxpqqqqqqzrgq77x",
    "nostr:nevent1qqstw4pcsv8aay4fg0w3qy6vpxl3su6t27g5zv3jmgk855c38ul8jvqpz4mhxue69uhhyetvv9ujuerpd46hxtnfduhsz2ngw368que69uhkjtnwdaehgu3wvf6kjmry9afkydj4fce42knyvyuk2u6kff4ju6nsvupzq2enk2kvd5d7ty56v7rk53a5sws8p3s3cpflqjy070c7fndf7fx9qvzqqqqqqyst0zrk",
    "nostr:nevent1qvzqqqqqqypzpdjfff6drz3dlglcpnkeltdwxkq8w9huuyr3un0pnckhg6mdsasxqyghwumn8ghj7vf5xqhxvdm69e5k7tcpzpmhxue69uhkztnwdaejumr0dshsz9mhwden5te0v9nk7unp9ehx7um5wgcjucm0d5hszrmhwden5te0vyhxummn9ekx7mqpzamhxue69uhkzarvv9ejumn0wd68ytnvv9hxgtcqypv46w5vz43qy98rgadx7rdm77hxhstgglxx7t3e7z488kalgku72d3wwv7"
];

pub const RELAYS: &'static [&'static str] = &[
    "wss://relay.damus.io",
    "wss://nostr.mom",
    "wss://nostr.oxtr.dev",
    "wss://nostr.bitcoiner.social",
    "wss://140.f7z.io",
    "wss://nos.lol",
    "wss://pyramid.fiatjaf.com",
    "wss://relay.snort.social",
    "wss://relay.primal.net",
];

#[derive(PartialEq, Debug)]
pub struct DayOneDelta {
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
}

impl DayOneDelta {
    pub fn to_seconds(&self) -> i64 {
        self.days * SECONDS_IN_A_DAY + self.hours * 60 * 60 + self.minutes * 60 + self.seconds as i64
    }
}

#[derive(PartialEq, Debug)]
pub enum DayOneContext<'a> {
    Before(&'a DayOneDelta),
    InaugurationDay(&'a DayOneDelta),
    DayOf(&'a DayOneDelta),
    After(&'a DayOneDelta),
}
