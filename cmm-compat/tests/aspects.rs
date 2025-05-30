use std::cell::LazyCell;

use calamine::{Data, DataType, Reader, Xlsx, open_workbook};
use cmm_compat::from_xlsx;
use cmm_core::{CMM, Domain, aspect::Aspect};

static XLSX: &str = "/home/bsiag.local/kli/Downloads/soc-cmm-2.3.4-basic_BSI.xlsx";
const CMM: LazyCell<CMM> = LazyCell::new(|| from_xlsx(XLSX).unwrap());
const OUTPUT: LazyCell<calamine::Range<Data>> = LazyCell::new(|| {
    open_workbook::<Xlsx<_>, _>(XLSX)
        .unwrap()
        .worksheet_range("_Output")
        .unwrap()
});

fn aspect(domain: Domain, index: usize) -> Aspect {
    CMM.aspects(&domain).unwrap().get(index).unwrap().clone()
}

fn score_at(absolute_position: (u32, u32)) -> Data {
    OUTPUT.get_value(absolute_position).unwrap().clone()
}

maturity!(Domain::Business, 1, 8);
maturity!(Domain::Business, 2, 26);
maturity!(Domain::Business, 3, 45);
maturity!(Domain::Business, 4, 79); // B 4.11 -> remove first occurence
maturity!(Domain::Business, 5, 90);

maturity!(Domain::People, 1, 103);
maturity!(Domain::People, 2, 144);
maturity!(Domain::People, 3, 157);
maturity!(Domain::People, 4, 176);
maturity!(Domain::People, 5, 197);

maturity!(Domain::Process, 1, 216);
// M 2.3.2 -> remove second
// M 2.3.5 -> remove second
// M 2.1.6 -> remove only
// M 2.3.9 -> remove only
maturity!(Domain::Process, 2, 247);
// M 3.9.1 -> remove only
// M 3.9.2 -> remove only
// M 3.9.3 -> remove only
maturity!(Domain::Process, 3, 275);
maturity!(Domain::Process, 4, 315);

capability!(Domain::Technology, 1, 371);
maturity!(Domain::Technology, 1, 372);
capability!(Domain::Technology, 2, 418);
maturity!(Domain::Technology, 2, 419);
capability!(Domain::Technology, 3, 472);
maturity!(Domain::Technology, 3, 473);
capability!(Domain::Technology, 4, 521);
maturity!(Domain::Technology, 4, 522);

capability!(Domain::Services, 1, 596);
maturity!(Domain::Services, 1, 597);
// S 2.16.33 -> remove only
// S 2.16.34 -> remove only
// S 2.16.35 -> remove only
// S 2.16.36 -> remove only
capability!(Domain::Services, 2, 675);
maturity!(Domain::Services, 2, 676);
capability!(Domain::Services, 3, 738);
maturity!(Domain::Services, 3, 739);
// S 4.14.25 -> remove only
// S 4.14.31 -> remove only
capability!(Domain::Services, 4, 799);
maturity!(Domain::Services, 4, 800);
capability!(Domain::Services, 5, 853);
maturity!(Domain::Services, 5, 854);
// S 6.14.20 -> remove only
capability!(Domain::Services, 6, 913);
maturity!(Domain::Services, 6, 914);
capability!(Domain::Services, 7, 966);
maturity!(Domain::Services, 7, 967);

#[macro_export]
macro_rules! maturity {
    ($domain:ty,$aspect:expr,$row:expr) => {
        compose_idents::compose_idents!(test_fn = [test_, $aspect, _, $row], {
            #[test]
            fn test_fn() {
                let aspect = aspect($domain, $aspect - 1);
                assert_eq!(
                    score_at(($row, 9)).as_i64().unwrap() as u8,
                    aspect.maturity_factor()
                );
                assert_eq!(
                    score_at(($row, 10)).as_i64().unwrap() as u8,
                    aspect.maturity_total_score()
                );
                assert_eq!(
                    score_at(($row, 11)).as_i64().unwrap() as u8,
                    aspect.maturity_max_score()
                );
                assert_eq!(
                    score_at(($row, 12)).as_f64().unwrap(),
                    aspect.maturity_final_score()
                );
            }
        });
    };
}

#[macro_export]
macro_rules! capability {
    ($domain:ty,$aspect:expr,$row:expr) => {
        compose_idents::compose_idents!(test_fn = [test_, $aspect, _, $row], {
            #[test]
            fn test_fn() {
                let aspect = aspect($domain, $aspect - 1);
                assert_eq!(
                    score_at(($row, 9)).as_i64().unwrap() as u8,
                    aspect.capability_factor()
                );
                assert_eq!(
                    score_at(($row, 10)).as_i64().unwrap() as u8,
                    aspect.capability_total_score()
                );
                assert_eq!(
                    score_at(($row, 11)).as_i64().unwrap() as u8,
                    aspect.capability_max_score()
                );
                assert_eq!(
                    score_at(($row, 12)).as_f64().unwrap(),
                    aspect.capability_final_score()
                );
            }
        });
    };
}
