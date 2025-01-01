/***
 * Types
 */

/// The `ArchitectureType` describes the type of chip architecture the node has
#[repr(u8)]
pub enum ArchitectureType {
    // https://github.com/docker-library/official-images#architectures-other-than-amd64
    Amd64 = 0,        // Linux x86-64
    Arm32v6 = 1,      // ARMv6 32-bit
    Arm32v7 = 2,      // ARMv7 32-bit
    Arm64v8 = 3,      // ARMv8 64-bit
    WindowsAmd64 = 4, // Windows x86-64
    Ppc64le = 5,      // IBM POWER8
    S390x = 6,        // IBM z Systems
    Mips64le = 7,     // MIPS64 LE
    Riscv64 = 8,      // RISC-V 64-bit
    I386 = 9,         // x86/i686
    Unknown = 255,
}

impl From<u8> for ArchitectureType {
    fn from(architecture_type: u8) -> Self {
        match architecture_type {
            0 => ArchitectureType::Amd64,
            1 => ArchitectureType::Arm32v6,
            2 => ArchitectureType::Arm32v7,
            3 => ArchitectureType::Arm64v8,
            4 => ArchitectureType::WindowsAmd64,
            5 => ArchitectureType::Ppc64le,
            6 => ArchitectureType::S390x,
            7 => ArchitectureType::Mips64le,
            8 => ArchitectureType::Riscv64,
            9 => ArchitectureType::I386,
            _ => ArchitectureType::Unknown,
        }
    }
}

/// The `CountryCode` represent the ISO code for a country
#[repr(u16)]
pub enum CountryCode {
    // https://www.iban.com/country-codes
    AD = 020, // Andorra
    AE = 784, // United Arab Emirates (the)
    AF = 004, // Afghanistan
    AG = 028, // Antigua and Barbuda
    AI = 660, // Anguilla
    AL = 008, // Albania
    AM = 051, // Armenia
    AO = 024, // Angola
    AQ = 010, // Antarctica
    AR = 032, // Argentina
    AS = 016, // American Samoa
    AT = 040, // Austria
    AU = 036, // Australia
    AW = 533, // Aruba
    AX = 248, // Åland Islands
    AZ = 031, // Azerbaijan
    BA = 070, // Bosnia and Herzegovina
    BB = 052, // Barbados
    BD = 050, // Bangladesh
    BE = 056, // Belgium
    BF = 854, // Burkina Faso
    BG = 100, // Bulgaria
    BH = 048, // Bahrain
    BI = 108, // Burundi
    BJ = 204, // Benin
    BL = 652, // Saint Barthélemy
    BM = 060, // Bermuda
    BN = 096, // Brunei Darussalam
    BO = 068, // Bolivia (Plurinational State of)
    BQ = 535, // Bonaire, Sint Eustatius and Saba
    BR = 076, // Brazil
    BS = 044, // Bahamas (the)
    BT = 064, // Bhutan
    BV = 074, // Bouvet Island
    BW = 072, // Botswana
    BY = 112, // Belarus
    BZ = 084, // Belize
    CA = 124, // Canada
    CC = 166, // Cocos (Keeling) Islands (the)
    CD = 180, // Congo (the Democratic Republic of the)
    CF = 140, // Central African Republic (the)
    CG = 178, // Congo (the)
    CH = 756, // Switzerland
    CI = 384, // Côte d'Ivoire
    CK = 184, // Cook Islands (the)
    CL = 152, // Chile
    CM = 120, // Cameroon
    CN = 156, // China
    CO = 170, // Colombia
    CR = 188, // Costa Rica
    CU = 192, // Cuba
    CV = 132, // Cabo Verde
    CW = 531, // Curaçao
    CX = 162, // Christmas Island
    CY = 196, // Cyprus
    CZ = 203, // Czechia
    DE = 276, // Germany
    DJ = 262, // Djibouti
    DK = 208, // Denmark
    DM = 212, // Dominica
    DO = 214, // Dominican Republic (the)
    DZ = 012, // Algeria
    EC = 218, // Ecuador
    EE = 233, // Estonia
    EG = 818, // Egypt
    EH = 732, // Western Sahara
    ER = 232, // Eritrea
    ES = 724, // Spain
    ET = 231, // Ethiopia
    FI = 246, // Finland
    FJ = 242, // Fiji
    FK = 238, // Falkland Islands (the) [Malvinas]
    FM = 583, // Micronesia (Federated States of)
    FO = 234, // Faroe Islands (the)
    FR = 250, // France
    GA = 266, // Gabon
    GB = 826, // United Kingdom of Great Britain and Northern Ireland (the)
    GD = 308, // Grenada
    GE = 268, // Georgia
    GF = 254, // French Guiana
    GG = 831, // Guernsey
    GH = 288, // Ghana
    GI = 292, // Gibraltar
    GL = 304, // Greenland
    GM = 270, // Gambia (the)
    GN = 324, // Guinea
    GP = 312, // Guadeloupe
    GQ = 226, // Equatorial Guinea
    GR = 300, // Greece
    GS = 239, // South Georgia and the South Sandwich Islands
    GT = 320, // Guatemala
    GU = 316, // Guam
    GW = 624, // Guinea-Bissau
    GY = 328, // Guyana
    HK = 344, // Hong Kong
    HM = 334, // Heard Island and McDonald Islands
    HN = 340, // Honduras
    HR = 191, // Croatia
    HT = 332, // Haiti
    HU = 348, // Hungary
    ID = 360, // Indonesia
    IE = 372, // Ireland
    IL = 376, // Israel
    IM = 833, // Isle of Man
    IN = 356, // India
    IO = 086, // British Indian Ocean Territory (the)
    IQ = 368, // Iraq
    IR = 364, // Iran (Islamic Republic of)
    IS = 352, // Iceland
    IT = 380, // Italy
    JE = 832, // Jersey
    JM = 388, // Jamaica
    JO = 400, // Jordan
    JP = 392, // Japan
    KE = 404, // Kenya
    KG = 417, // Kyrgyzstan
    KH = 116, // Cambodia
    KI = 296, // Kiribati
    KM = 174, // Comoros (the)
    KN = 659, // Saint Kitts and Nevis
    KP = 408, // Korea (the Democratic People's Republic of)
    KR = 410, // Korea (the Republic of)
    KW = 414, // Kuwait
    KY = 136, // Cayman Islands (the)
    KZ = 398, // Kazakhstan
    LA = 418, // Lao People's Democratic Republic (the)
    LB = 422, // Lebanon
    LC = 662, // Saint Lucia
    LI = 438, // Liechtenstein
    LK = 144, // Sri Lanka
    LR = 430, // Liberia
    LS = 426, // Lesotho
    LT = 440, // Lithuania
    LU = 442, // Luxembourg
    LV = 428, // Latvia
    LY = 434, // Libya
    MA = 504, // Morocco
    MC = 492, // Monaco
    MD = 498, // Moldova (the Republic of)
    ME = 499, // Montenegro
    MF = 663, // Saint Martin (French part)
    MG = 450, // Madagascar
    MH = 584, // Marshall Islands (the)
    MK = 807, // Republic of North Macedonia
    ML = 466, // Mali
    MM = 104, // Myanmar
    MN = 496, // Mongolia
    MO = 446, // Macao
    MP = 580, // Northern Mariana Islands (the)
    MQ = 474, // Martinique
    MR = 478, // Mauritania
    MS = 500, // Montserrat
    MT = 470, // Malta
    MU = 480, // Mauritius
    MV = 462, // Maldives
    MW = 454, // Malawi
    MX = 484, // Mexico
    MY = 458, // Malaysia
    MZ = 508, // Mozambique
    NA = 516, // Namibia
    NC = 540, // New Caledonia
    NE = 562, // Niger (the)
    NF = 574, // Norfolk Island
    NG = 566, // Nigeria
    NI = 558, // Nicaragua
    NL = 528, // Netherlands (the)
    NO = 578, // Norway
    NP = 524, // Nepal
    NR = 520, // Nauru
    NU = 570, // Niue
    NZ = 554, // New Zealand
    OM = 512, // Oman
    PA = 591, // Panama
    PE = 604, // Peru
    PF = 258, // French Polynesia
    PG = 598, // Papua New Guinea
    PH = 608, // Philippines (the)
    PK = 586, // Pakistan
    PL = 616, // Poland
    PM = 666, // Saint Pierre and Miquelon
    PN = 612, // Pitcairn
    PR = 630, // Puerto Rico
    PS = 275, // Palestine, State of
    PT = 620, // Portugal
    PW = 585, // Palau
    PY = 600, // Paraguay
    QA = 634, // Qatar
    RE = 638, // Réunion
    RO = 642, // Romania
    RS = 688, // Serbia
    RU = 643, // Russian Federation (the)
    RW = 646, // Rwanda
    SA = 682, // Saudi Arabia
    SB = 090, // Solomon Islands
    SC = 690, // Seychelles
    SD = 729, // Sudan (the)
    SE = 752, // Sweden
    SG = 702, // Singapore
    SH = 654, // Saint Helena, Ascension and Tristan da Cunha
    SI = 705, // Slovenia
    SJ = 744, // Svalbard and Jan Mayen
    SK = 703, // Slovakia
    SL = 694, // Sierra Leone
    SM = 674, // San Marino
    SN = 686, // Senegal
    SO = 706, // Somalia
    SR = 740, // Suriname
    SS = 728, // South Sudan
    ST = 678, // Sao Tome and Principe
    SV = 222, // El Salvador
    SX = 534, // Sint Maarten (Dutch part)
    SY = 760, // Syrian Arab Republic
    SZ = 748, // Eswatini
    TC = 796, // Turks and Caicos Islands (the)
    TD = 148, // Chad
    TF = 260, // French Southern Territories (the)
    TG = 768, // Togo
    TH = 764, // Thailand
    TJ = 762, // Tajikistan
    TK = 772, // Tokelau
    TL = 626, // Timor-Leste
    TM = 795, // Turkmenistan
    TN = 788, // Tunisia
    TO = 776, // Tonga
    TR = 792, // Turkey
    TT = 780, // Trinidad and Tobago
    TV = 798, // Tuvalu
    TW = 158, // Taiwan (Province of China)
    TZ = 834, // Tanzania, United Republic of
    UA = 804, // Ukraine
    UG = 800, // Uganda
    UM = 581, // United States Minor Outlying Islands (the)
    US = 840, // United States of America (the)
    UY = 858, // Uruguay
    UZ = 860, // Uzbekistan
    VA = 336, // Holy See (the)
    VC = 670, // Saint Vincent and the Grenadines
    VE = 862, // Venezuela (Bolivarian Republic of)
    VG = 092, // Virgin Islands (British)
    VI = 850, // Virgin Islands (U.S.)
    VN = 704, // Viet Nam
    VU = 548, // Vanuatu
    WF = 876, // Wallis and Futuna
    WS = 882, // Samoa
    YE = 887, // Yemen
    YT = 175, // Mayotte
    ZA = 710, // South Africa
    ZM = 894, // Zambia
    ZW = 716, // Zimbabwe

    Unknown = 999,
}

impl From<u16> for CountryCode {
    fn from(country_code: u16) -> Self {
        match country_code {
            020 => CountryCode::AD,
            784 => CountryCode::AE,
            004 => CountryCode::AF,
            028 => CountryCode::AG,
            660 => CountryCode::AI,
            008 => CountryCode::AL,
            051 => CountryCode::AM,
            024 => CountryCode::AO,
            010 => CountryCode::AQ,
            032 => CountryCode::AR,
            016 => CountryCode::AS,
            040 => CountryCode::AT,
            036 => CountryCode::AU,
            533 => CountryCode::AW,
            248 => CountryCode::AX,
            031 => CountryCode::AZ,
            070 => CountryCode::BA,
            052 => CountryCode::BB,
            050 => CountryCode::BD,
            056 => CountryCode::BE,
            854 => CountryCode::BF,
            100 => CountryCode::BG,
            048 => CountryCode::BH,
            108 => CountryCode::BI,
            204 => CountryCode::BJ,
            652 => CountryCode::BL,
            060 => CountryCode::BM,
            096 => CountryCode::BN,
            068 => CountryCode::BO,
            535 => CountryCode::BQ,
            076 => CountryCode::BR,
            044 => CountryCode::BS,
            064 => CountryCode::BT,
            074 => CountryCode::BV,
            072 => CountryCode::BW,
            112 => CountryCode::BY,
            084 => CountryCode::BZ,
            124 => CountryCode::CA,
            166 => CountryCode::CC,
            180 => CountryCode::CD,
            140 => CountryCode::CF,
            178 => CountryCode::CG,
            756 => CountryCode::CH,
            384 => CountryCode::CI,
            184 => CountryCode::CK,
            152 => CountryCode::CL,
            120 => CountryCode::CM,
            156 => CountryCode::CN,
            170 => CountryCode::CO,
            188 => CountryCode::CR,
            192 => CountryCode::CU,
            132 => CountryCode::CV,
            531 => CountryCode::CW,
            162 => CountryCode::CX,
            196 => CountryCode::CY,
            203 => CountryCode::CZ,
            276 => CountryCode::DE,
            262 => CountryCode::DJ,
            208 => CountryCode::DK,
            212 => CountryCode::DM,
            214 => CountryCode::DO,
            012 => CountryCode::DZ,
            218 => CountryCode::EC,
            233 => CountryCode::EE,
            818 => CountryCode::EG,
            732 => CountryCode::EH,
            232 => CountryCode::ER,
            724 => CountryCode::ES,
            231 => CountryCode::ET,
            246 => CountryCode::FI,
            242 => CountryCode::FJ,
            238 => CountryCode::FK,
            583 => CountryCode::FM,
            234 => CountryCode::FO,
            250 => CountryCode::FR,
            266 => CountryCode::GA,
            826 => CountryCode::GB,
            308 => CountryCode::GD,
            268 => CountryCode::GE,
            254 => CountryCode::GF,
            831 => CountryCode::GG,
            288 => CountryCode::GH,
            292 => CountryCode::GI,
            304 => CountryCode::GL,
            270 => CountryCode::GM,
            324 => CountryCode::GN,
            312 => CountryCode::GP,
            226 => CountryCode::GQ,
            300 => CountryCode::GR,
            239 => CountryCode::GS,
            320 => CountryCode::GT,
            316 => CountryCode::GU,
            624 => CountryCode::GW,
            328 => CountryCode::GY,
            344 => CountryCode::HK,
            334 => CountryCode::HM,
            340 => CountryCode::HN,
            191 => CountryCode::HR,
            332 => CountryCode::HT,
            348 => CountryCode::HU,
            360 => CountryCode::ID,
            372 => CountryCode::IE,
            376 => CountryCode::IL,
            833 => CountryCode::IM,
            356 => CountryCode::IN,
            086 => CountryCode::IO,
            368 => CountryCode::IQ,
            364 => CountryCode::IR,
            352 => CountryCode::IS,
            380 => CountryCode::IT,
            832 => CountryCode::JE,
            388 => CountryCode::JM,
            400 => CountryCode::JO,
            392 => CountryCode::JP,
            404 => CountryCode::KE,
            417 => CountryCode::KG,
            116 => CountryCode::KH,
            296 => CountryCode::KI,
            174 => CountryCode::KM,
            659 => CountryCode::KN,
            408 => CountryCode::KP,
            410 => CountryCode::KR,
            414 => CountryCode::KW,
            136 => CountryCode::KY,
            398 => CountryCode::KZ,
            418 => CountryCode::LA,
            422 => CountryCode::LB,
            662 => CountryCode::LC,
            438 => CountryCode::LI,
            144 => CountryCode::LK,
            430 => CountryCode::LR,
            426 => CountryCode::LS,
            440 => CountryCode::LT,
            442 => CountryCode::LU,
            428 => CountryCode::LV,
            434 => CountryCode::LY,
            504 => CountryCode::MA,
            492 => CountryCode::MC,
            498 => CountryCode::MD,
            499 => CountryCode::ME,
            663 => CountryCode::MF,
            450 => CountryCode::MG,
            584 => CountryCode::MH,
            807 => CountryCode::MK,
            466 => CountryCode::ML,
            104 => CountryCode::MM,
            496 => CountryCode::MN,
            446 => CountryCode::MO,
            580 => CountryCode::MP,
            474 => CountryCode::MQ,
            478 => CountryCode::MR,
            500 => CountryCode::MS,
            470 => CountryCode::MT,
            480 => CountryCode::MU,
            462 => CountryCode::MV,
            454 => CountryCode::MW,
            484 => CountryCode::MX,
            458 => CountryCode::MY,
            508 => CountryCode::MZ,
            516 => CountryCode::NA,
            540 => CountryCode::NC,
            562 => CountryCode::NE,
            574 => CountryCode::NF,
            566 => CountryCode::NG,
            558 => CountryCode::NI,
            528 => CountryCode::NL,
            578 => CountryCode::NO,
            524 => CountryCode::NP,
            520 => CountryCode::NR,
            570 => CountryCode::NU,
            554 => CountryCode::NZ,
            512 => CountryCode::OM,
            591 => CountryCode::PA,
            604 => CountryCode::PE,
            258 => CountryCode::PF,
            598 => CountryCode::PG,
            608 => CountryCode::PH,
            586 => CountryCode::PK,
            616 => CountryCode::PL,
            666 => CountryCode::PM,
            612 => CountryCode::PN,
            630 => CountryCode::PR,
            275 => CountryCode::PS,
            620 => CountryCode::PT,
            585 => CountryCode::PW,
            600 => CountryCode::PY,
            634 => CountryCode::QA,
            638 => CountryCode::RE,
            642 => CountryCode::RO,
            688 => CountryCode::RS,
            643 => CountryCode::RU,
            646 => CountryCode::RW,
            682 => CountryCode::SA,
            090 => CountryCode::SB,
            690 => CountryCode::SC,
            729 => CountryCode::SD,
            752 => CountryCode::SE,
            702 => CountryCode::SG,
            654 => CountryCode::SH,
            705 => CountryCode::SI,
            744 => CountryCode::SJ,
            703 => CountryCode::SK,
            694 => CountryCode::SL,
            674 => CountryCode::SM,
            686 => CountryCode::SN,
            706 => CountryCode::SO,
            740 => CountryCode::SR,
            728 => CountryCode::SS,
            678 => CountryCode::ST,
            222 => CountryCode::SV,
            534 => CountryCode::SX,
            760 => CountryCode::SY,
            748 => CountryCode::SZ,
            796 => CountryCode::TC,
            148 => CountryCode::TD,
            260 => CountryCode::TF,
            768 => CountryCode::TG,
            764 => CountryCode::TH,
            762 => CountryCode::TJ,
            772 => CountryCode::TK,
            626 => CountryCode::TL,
            795 => CountryCode::TM,
            788 => CountryCode::TN,
            776 => CountryCode::TO,
            792 => CountryCode::TR,
            780 => CountryCode::TT,
            798 => CountryCode::TV,
            158 => CountryCode::TW,
            834 => CountryCode::TZ,
            804 => CountryCode::UA,
            800 => CountryCode::UG,
            581 => CountryCode::UM,
            840 => CountryCode::US,
            858 => CountryCode::UY,
            860 => CountryCode::UZ,
            336 => CountryCode::VA,
            670 => CountryCode::VC,
            862 => CountryCode::VE,
            092 => CountryCode::VG,
            850 => CountryCode::VI,
            704 => CountryCode::VN,
            548 => CountryCode::VU,
            876 => CountryCode::WF,
            882 => CountryCode::WS,
            887 => CountryCode::YE,
            175 => CountryCode::YT,
            710 => CountryCode::ZA,
            894 => CountryCode::ZM,
            716 => CountryCode::ZW,
            _ => CountryCode::Unknown,
        }
    }
}
