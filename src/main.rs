

fn main() {
    let text = [
        "И куропатку на грушевом дереве",
        "Двух голубей",
        "Три французские курицы",
        "Четыре щебечущих пташек",
        "Пять золотых колец",
        "Шесть несущихся гусей",
        "Семь плавающих лебедей",
        "Восемь доящих доярок",
        "Девять танцующих дам",
        "Десять прыгающих господ",
        "Одиннадцать играющих волынщиков",
        "Двенадцать барабанящих барабанщиков",
    ];

    let days = [
        "первый",
        "второй",
        "третий",
        "четвертый",
        "пятый",
        "шестой",
        "седьмой",
        "восьмой",
        "девятый",
        "десятый",
        "одиннадцатый",
        "двенадцатый",
    ];


    for i in 0..text.len() {

        let next = i + 1;
        let day = days[i];

        if day != "" {
            println!("{} {} день Рождества", if day == days[1] { "Во" } else { "В" }, day);
            println!("Моя любимая отправила мне:");

            for idx in (0..next).rev() {
                let mut concrete = String::from(text[idx]);
                if i < 1 {
                    concrete = concrete.replace("И к", "К");
                }
                println!("{}", concrete);
            }

            println!(" ")
        }
    }
}
