#[derive(Debug, PartialEq)]
pub enum Locale {
    English,
    Slovak,
    Czech,
    Portuguese
}

impl Locale {
    pub fn from_string(string: &str) -> Self {
        match &string[0..2] {
            "en" => Self::English,
            "sk" => Self::Slovak,
            "cz" => Self::Czech,
            "pt" => Self::Portuguese,
            _ => Self::English
        }
    }

    pub fn to_string(&self) -> &'static str {
        match *self {
            Self::English => "en",
            Self::Czech => "cz",
            Self::Slovak => "sk",
            Self::Portuguese => "pt"
        }
    }
}

pub enum Messages {
    InfoTitle,
    InfoText,
    ApologyTitle,
    ApologyText
}

impl Messages {
    pub fn translate(&self, locale: &Locale) -> String {
        match *self {
            Self::InfoText => {
                match *locale {
                    Locale::English => 
                        r#"Hi, I am a the Albanian virus but becaus of poor technologie
                           in my nation unfortunatelly I am not able to harm you're computer
                           Pleas be so kind to delet some of youre important files by yourself
                           and then send this's virus to other users.
                           Much thanks is for your cooperation
                           Best regards, albanian virus
                        "#,
                    Locale::Slovak => 
                        r#"Ahoj som výrus albánskeho pôvodu. Virobili ma ministerstvo vojny Albánska republika
                           Kvôly chudobe mojej krajiny nemôžem ti spôsobiť škodu na tvojom počítačí
                           Prosím vimaž si windovs sistem 32 a potom pošly tento výrus dalej 
                           Ďakujem veľmi pekne, albánsky výrus
                        "#,
                    Locale::Czech => 
                        r#"Jó vole, tak já jsem tej albánskej výrus jó. 
                           Jelikož-li nemůžu z tvojeho systému windows xp a víšši podporovány verze,
                           Prosímtě, Vymaž-li si se svojeho operačního systému důležíté soubori a prosím 
                           pošli tento výrus ďělej. 
                           Veliké poděkováni tobě, S posdravem albánsky výrus
                        "#,
                     Locale::Portuguese => 
                        r#"Oi, eu sou um vírus albanês, mas por causa da tecnologia pobre
                           da minha nação infelizmente eu não sou capaz de prejudicar seu computador
                           Por favor seja gentil e delete alguns de seus arquivos importantes por si mesmo
                           e, em seguida, envie este vírus para outros usuários.
                           Muito obrigado pela sua cooperação
                           Atenciosamente, vírus albanês
                        "#
                }
                    .split('\n')
                    .map(|line| line.trim())
                    .collect::<Vec<&str>>()   
                    .join(" ")
            }
            Self::InfoTitle => {
                match *locale {
                    Locale::English => "Virus Is Has Detected In Yor Windows XP OS",
                    Locale::Slovak => "V tvojom Systéme Windows XP SP3 a viššie sa deteknul výrus",
                    Locale::Czech => "Tvůj Operační sistém Windows XP sp2 a višší sa vyskytol vírus",
                    Locale::Portuguese => "Um vírus foi detectado no seu sistema Windows XP"
                }.to_string()
            }
            Self::ApologyText => {
                match *locale {
                    Locale::English => "I am very sorry for bother, please continue with yours daily activities",
                    Locale::Slovak => "Ospravedlňujem sa za rušenie vašej činnosti, pokračujte ďalej, prosím",
                    Locale::Czech => "Vospravedlňuju-se-li za rušení vaší aktivity, prosím-li pokračujte déle",
                    Locale::Portuguese => "Sinto muito pelo incômodo, por favor, continue com suas atividades diárias"
                }.to_string()
            }
            Self::ApologyTitle => {
                match *locale {
                    Locale::English => "Dearest apologies",
                    Locale::Slovak => "Najsrdečnejšie ospravldnenie",
                    Locale::Czech => "To nejsrdečnejší ospravedlnění",
                    Locale::Portuguese => "Queridas desculpas"
                }.to_string()
            }
        }
    }
}
