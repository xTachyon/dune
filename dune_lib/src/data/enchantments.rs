#[derive(Debug)]
pub enum Enchantment {
    Protection = 0,
    FireProtection = 1,
    FeatherFalling = 2,
    BlastProtection = 3,
    ProjectileProtection = 4,
    Respiration = 5,
    AquaAffinity = 6,
    Thorns = 7,
    DepthStrider = 8,
    FrostWalker = 9,
    BindingCurse = 10,
    SoulSpeed = 11,
    Sharpness = 12,
    Smite = 13,
    BaneOfArthropods = 14,
    Knockback = 15,
    FireAspect = 16,
    Looting = 17,
    Sweeping = 18,
    Efficiency = 19,
    SilkTouch = 20,
    Unbreaking = 21,
    Fortune = 22,
    Power = 23,
    Punch = 24,
    Flame = 25,
    Infinity = 26,
    LuckOfTheSea = 27,
    Lure = 28,
    Loyalty = 29,
    Impaling = 30,
    Riptide = 31,
    Channeling = 32,
    Multishot = 33,
    QuickCharge = 34,
    Piercing = 35,
    Mending = 36,
    VanishingCurse = 37,
}
impl Enchantment {
    pub fn from(input: &str) -> anyhow::Result<Self> {
        let minecraft = "minecraft:";
        let s;
        if input.starts_with(minecraft) {
            s = &input[minecraft.len()..];
        } else {
            anyhow::bail!("unknown enchantment: {}", input);
        }
        use Enchantment::*;

        let result = match s {
            "protection" => Protection,
            "fire_protection" => FireProtection,
            "feather_falling" => FeatherFalling,
            "blast_protection" => BlastProtection,
            "projectile_protection" => ProjectileProtection,
            "respiration" => Respiration,
            "aqua_affinity" => AquaAffinity,
            "thorns" => Thorns,
            "depth_strider" => DepthStrider,
            "frost_walker" => FrostWalker,
            "binding_curse" => BindingCurse,
            "soul_speed" => SoulSpeed,
            "sharpness" => Sharpness,
            "smite" => Smite,
            "bane_of_arthropods" => BaneOfArthropods,
            "knockback" => Knockback,
            "fire_aspect" => FireAspect,
            "looting" => Looting,
            "sweeping" => Sweeping,
            "efficiency" => Efficiency,
            "silk_touch" => SilkTouch,
            "unbreaking" => Unbreaking,
            "fortune" => Fortune,
            "power" => Power,
            "punch" => Punch,
            "flame" => Flame,
            "infinity" => Infinity,
            "luck_of_the_sea" => LuckOfTheSea,
            "lure" => Lure,
            "loyalty" => Loyalty,
            "impaling" => Impaling,
            "riptide" => Riptide,
            "channeling" => Channeling,
            "multishot" => Multishot,
            "quick_charge" => QuickCharge,
            "piercing" => Piercing,
            "mending" => Mending,
            "vanishing_curse" => VanishingCurse,
            _ => anyhow::bail!("unknown enchantment: {}", input),
        };
        Ok(result)
    }
}
