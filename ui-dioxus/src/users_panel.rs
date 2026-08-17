use data_models::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn UsersPanel() -> Element {
    let mut rps_value = use_signal(|| 0.0);
    let mut p_passports = use_signal(Proportion::none);
    let mut p_dl = use_signal(Proportion::none);

    let passport_percent = format!("{:.0}%", *p_passports() * 100.0);
    let dl_percent = format!("{:.0}%", *p_dl() * 100.0);

    rsx! {
        form {
            label {
                "User Journeys per Second: {rps_value}"
                input {
                    name: "request_per_second",
                    r#type: "range",
                    min: "0",
                    max: "20",
                    value: "{rps_value}",
                    oninput: move |e| *rps_value.write() = e.value().parse().unwrap(),
                }
            }
            label {
                "Passport: {passport_percent}"
                input {
                    name: "passport",
                    r#type: "range",
                    min: "0",
                    max: "1",
                    step: "0.01",
                    value: "0",
                    oninput: move |e| *p_passports.write() = e.value().parse::<f64>().unwrap().to_saturated_proportion(),
                }
            }
            label {
                "Driving Licence: {dl_percent}"
                input {
                    name: "driving_license",
                    r#type: "range",
                    min: "0",
                    max: "1",
                    step: "0.01",
                    value: "0",
                    oninput: move |e| *p_dl.write() = e.value().parse::<f64>().unwrap().to_saturated_proportion(),
                }
            }
        }
    }
}
