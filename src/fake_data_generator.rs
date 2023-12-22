use fake::Fake;
use fake::faker::address::en::*;
use fake::faker::company::en::CompanyName;
use fake::faker::internet::en::SafeEmail;
use fake::faker::lorem::en::Paragraph;
use fake::faker::name::en::*;
use fake::faker::phone_number::raw::PhoneNumber;
use fake::locales::*;

pub struct Employee {
    id: u32,
    company: String,
    name: String,
    age: u8,
    address: String,
    phone: String,
    email: String,
    introduction: String,
}

pub fn generate_fake_data() -> String {
    let employee = Employee {
        id: (1..1_000_000_000).fake(),
        company: CompanyName().fake(),
        name: Name().fake(),
        age: (1..65).fake(),
        address: CityName().fake(),
        phone: PhoneNumber(EN).fake(),
        email: SafeEmail().fake(),
        introduction: Paragraph(30..31).fake(),
    };

    // 產生 JSON 格式的資料
    let message = format!("{{ \
            \"id\":\"{}\", \
            \"company\":\"{}\", \
            \"name\":\"{}\", \
            \"age\":\"{}\", \
            \"address\":\"{}\", \
            \"phone\":\"{}\", \
            \"email\":\"{}\", \
            \"introduction\":\"{}\" \
        }}",
                          employee.id,
                          employee.company,
                          employee.name,
                          employee.age,
                          employee.address,
                          employee.phone,
                          employee.email,
                          employee.introduction
    );

    message
}