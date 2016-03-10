use std::ptr::read;
use ::exec::query::FileScan;
use ::exec::gen_plan::{
    gen_update_plan,
};
use ::utils::pointer::read_string;
use super::test_query::gen_test_manager;


#[test]
fn test_gen_update_plan() {
    {
        let table_name = "test_gen_plan_message".to_string();
        let manager = gen_test_manager(&table_name);
        let mut update = gen_plan_helper!(
            "update test_gen_plan_message set score = 86.86, content = \"updated\"", &manager);
        update.open();
        assert_pattern!(update.get_next(), Some(..));
        assert_pattern!(update.get_next(), Some(..));
        assert_pattern!(update.get_next(), Some(..));
        assert_pattern!(update.get_next(), None);

        let mut scan = FileScan::new(&table_name, &manager);
        scan.open();
        let t1 = extract!(scan.get_next(), Some(tuple_data), tuple_data);
        let t2 = extract!(scan.get_next(), Some(tuple_data), tuple_data);
        let t3 = extract!(scan.get_next(), Some(tuple_data), tuple_data);
        assert_pattern!(scan.get_next(), None);
        assert_eq!(unsafe{ read::<f32>(t1[1] as *const f32) }, 86.86);
        assert_eq!(unsafe{ read::<f32>(t2[1] as *const f32) }, 86.86);
        assert_eq!(unsafe{ read::<f32>(t3[1] as *const f32) }, 86.86);
        assert_eq!(unsafe{ read_string(t1[2], 16) }, "updated");
        assert_eq!(unsafe{ read_string(t2[2], 16) }, "updated");
        assert_eq!(unsafe{ read_string(t3[2], 16) }, "updated");
    }
    {
        let table_name = "test_gen_plan_message".to_string();
        let manager = gen_test_manager(&table_name);
        let mut update = gen_plan_helper!(
            "update test_gen_plan_message set score = 86.86 where id = 777",
            &manager);
        update.open();
        assert_pattern!(update.get_next(), Some(..));
        assert_pattern!(update.get_next(), None);

        let mut scan = FileScan::new(&table_name, &manager);
        scan.open();
        assert_pattern!(scan.get_next(), Some(..));
        let t2 = extract!(scan.get_next(), Some(tuple_data), tuple_data);
        assert_pattern!(scan.get_next(), Some(..));
        assert_pattern!(scan.get_next(), None);
        assert_eq!(unsafe{ read::<f32>(t2[1] as *const f32) }, 86.86);
    }
}
