use lemon_mbl::enums::monster_flags::MonsterFlag;
use lemon_mbl::get_game_data;
use lemon_mbl::traits::has_id::HasId;
use lemon_mbl::traits::has_internal_name::HasInternalName;

#[test]
fn test_monster_data() {
    let game_data = get_game_data();
    let test_monster = game_data.monsters.get(0).unwrap();
    assert_eq!(test_monster.internal_name(), "test_monster");
    assert_eq!(test_monster.id(), 0);
    assert_eq!(test_monster.vitality(), 50);
    assert_eq!(test_monster.potential(), 60);
    assert_eq!(test_monster.control(), 10);
    assert_eq!(test_monster.strength(), 13);
    assert_eq!(test_monster.resilience(), 14);
    assert_eq!(test_monster.speed(), 15);
    assert_eq!(test_monster.technique(), 16);
    assert_eq!(test_monster.agility(), 17);
    assert_eq!(test_monster.vigilance(), 6000);
    assert_eq!(test_monster.focus(), 19);
    assert_eq!(test_monster.flags().len(), 1);
    assert!(test_monster.has_flag(MonsterFlag::Flying))
}