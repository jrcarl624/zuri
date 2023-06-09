use crate::proto::ints::VarU32;
use num_derive::{FromPrimitive, ToPrimitive};
use zuri_net_derive::proto;

#[proto(VarU32)]
#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum SoundEvent {
    ItemUseOn,
    Hit,
    Step,
    Fly,
    Jump,
    Break,
    Place,
    HeavyStep,
    Gallop,
    Fall,
    Ambient,
    AmbientBaby,
    AmbientInWater,
    Breathe,
    Death,
    DeathInWater,
    DeathToZombie,
    Hurt,
    HurtInWater,
    Mad,
    Boost,
    Bow,
    SquishBig,
    SquishSmall,
    FallBig,
    FallSmall,
    Splash,
    Fizz,
    Flap,
    Swim,
    Drink,
    Eat,
    Takeoff,
    Shake,
    Plop,
    Land,
    Saddle,
    Armor,
    ArmorPlace,
    AddChest,
    Throw,
    Attack,
    AttackNoDamage,
    AttackStrong,
    Warn,
    Shear,
    Milk,
    Thunder,
    Explode,
    Fire,
    Ignite,
    Fuse,
    Stare,
    Spawn,
    Shoot,
    BreakBlock,
    Launch,
    Blast,
    LargeBlast,
    Twinkle,
    Remedy,
    Unfect,
    LevelUp,
    BowHit,
    BulletHit,
    ExtinguishFire,
    ItemFizz,
    ChestOpen,
    ChestClosed,
    ShulkerBoxOpen,
    ShulkerBoxClosed,
    EnderChestOpen,
    EnderChestClosed,
    PowerOn,
    PowerOff,
    Attach,
    Detach,
    Deny,
    Tripod,
    Pop,
    DropSlot,
    Note,
    Thorns,
    PistonIn,
    PistonOut,
    Portal,
    Water,
    LavaPop,
    Lava,
    Burp,
    BucketFillWater,
    BucketFillLava,
    BucketEmptyWater,
    BucketEmptyLava,
    EquipChain,
    EquipDiamond,
    EquipGeneric,
    EquipGold,
    EquipIron,
    EquipLeather,
    EquipElytra,
    Record13,
    RecordCat,
    RecordBlocks,
    RecordChirp,
    RecordFar,
    RecordMall,
    RecordMellohi,
    RecordStal,
    RecordStrad,
    RecordWard,
    Record11,
    RecordWait,
    RecordNull,
    Flop,
    GuardianCurse,
    MobWarning,
    MobWarningBaby,
    Teleport,
    ShulkerOpen,
    ShulkerClose,
    Haggle,
    HaggleYes,
    HaggleNo,
    HaggleIdle,
    ChorusGrow,
    ChorusDeath,
    Glass,
    PotionBrewed,
    CastSpell,
    PrepareAttackSpell,
    PrepareSummon,
    PrepareWololo,
    Fang,
    Charge,
    TakePicture,
    PlaceLeashKnot,
    BreakLeashKnot,
    AmbientGrowl,
    AmbientWhine,
    AmbientPant,
    AmbientPurr,
    AmbientPurreow,
    DeathMinVolume,
    DeathMidVolume,
    ImitateBlaze,
    ImitateCaveSpider,
    ImitateCreeper,
    ImitateElderGuardian,
    ImitateEnderDragon,
    ImitateEnderman,
    ImitateEndermite,
    ImitateEvocationIllager,
    ImitateGhast,
    ImitateHusk,
    ImitateIllusionIllager,
    ImitateMagmaCube,
    ImitatePolarBear,
    ImitateShulker,
    ImitateSilverfish,
    ImitateSkeleton,
    ImitateSlime,
    ImitateSpider,
    ImitateStray,
    ImitateVex,
    ImitateVindicationIllager,
    ImitateWitch,
    ImitateWither,
    ImitateWitherSkeleton,
    ImitateWolf,
    ImitateZombie,
    ImitateZombiePigman,
    ImitateZombieVillager,
    EnderEyePlaced,
    EndPortalCreated,
    AnvilUse,
    BottleDragonBreath,
    PortalTravel,
    TridentHit,
    TridentReturn,
    TridentRiptide1,
    TridentRiptide2,
    TridentRiptide3,
    TridentThrow,
    TridentThunder,
    TridentHitGround,
    Default,
    FletchingTableUse,
    ElemConstructOpen,
    IceBombHit,
    BalloonPop,
    LtReactionIceBomb,
    LtReactionBleach,
    LtReactionElephantToothpaste,
    LtReactionElephantToothpaste2,
    LtReactionGlowStick,
    LtReactionGlowStick2,
    LtReactionLuminol,
    LtReactionSalt,
    LtReactionFertilizer,
    LtReactionFireball,
    LtReactionMagnesiumSalt,
    LtReactionMiscFire,
    LtReactionFire,
    LtReactionMiscExplosion,
    LtReactionMiscMystical,
    LtReactionMiscMystical2,
    LtReactionProduct,
    SparklerUse,
    GlowStickUse,
    SparklerActive,
    ConvertToDrowned,
    BucketFillFish,
    BucketEmptyFish,
    BubbleColumnUpwards,
    BubbleColumnDownwards,
    BubblePop,
    BubbleUpInside,
    BubbleDownInside,
    HurtBaby,
    DeathBaby,
    StepBaby,
    SpawnBaby,
    Born,
    TurtleEggBreak,
    TurtleEggCrack,
    TurtleEggHatched,
    LayEgg,
    TurtleEggAttacked,
    BeaconActivate,
    BeaconAmbient,
    BeaconDeactivate,
    BeaconPower,
    ConduitActivate,
    ConduitAmbient,
    ConduitAttack,
    ConduitDeactivate,
    ConduitShort,
    Swoop,
    BambooSaplingPlace,
    PreSneeze,
    Sneeze,
    AmbientTame,
    Scared,
    ScaffoldingClimb,
    CrossbowLoadingStart,
    CrossbowLoadingMiddle,
    CrossbowLoadingEnd,
    CrossbowShoot,
    CrossbowQuickChargeStart,
    CrossbowQuickChargeMiddle,
    CrossbowQuickChargeEnd,
    AmbientAggressive,
    AmbientWorried,
    CantBreed,
    ShieldBlock,
    LecternBookPlace,
    GrindstoneUse,
    Bell,
    CampfireCrackle,
    Roar,
    Stun,
    SweetBerryBushHurt,
    SweetBerryBushPick,
    CartographyTableUse,
    StonecutterUse,
    ComposterEmpty,
    ComposterFill,
    ComposterFillLayer,
    ComposterReady,
    BarrelOpen,
    BarrelClose,
    RaidHorn,
    LoomUse,
    AmbientInRaid,
    UicartographyTableUse,
    UistonecutterUse,
    UiloomUse,
    SmokerUse,
    BlastFurnaceUse,
    SmithingTableUse,
    Screech,
    Sleep,
    FurnaceUse,
    MooshroomConvert,
    MilkSuspiciously,
    Celebrate,
    JumpPrevent,
    AmbientPollinate,
    BeehiveDrip,
    BeehiveEnter,
    BeehiveExit,
    BeehiveWork,
    BeehiveShear,
    HoneybottleDrink,
    AmbientCave,
    Retreat,
    ConvertToZombified,
    Admire,
    StepLava,
    Tempt,
    Panic,
    Angry,
    AmbientMoodWarpedForest,
    AmbientMoodSoulsandValley,
    AmbientMoodNetherWastes,
    AmbientMoodBasaltDeltas,
    AmbientMoodCrimsonForest,
    RespawnAnchorCharge,
    RespawnAnchorDeplete,
    RespawnAnchorSetSpawn,
    RespawnAnchorAmbient,
    SoulEscapeQuiet,
    SoulEscapeLoud,
    RecordPigstep,
    LinkCompassToLodestone,
    UseSmithingTable,
    EquipNetherite,
    AmbientLoopWarpedForest,
    AmbientLoopSoulsandValley,
    AmbientLoopNetherWastes,
    AmbientLoopBasaltDeltas,
    AmbientLoopCrimsonForest,
    AmbientAdditionWarpedForest,
    AmbientAdditionSoulsandValley,
    AmbientAdditionNetherWastes,
    AmbientAdditionBasaltDeltas,
    AmbientAdditionCrimsonForest,
    SculkSensorPowerOn,
    SculkSensorPowerOff,
    BucketFillPowderSnow,
    BucketEmptyPowderSnow,
    PointedDripstoneCauldronDripWater,
    PointedDripstoneCauldronDripLava,
    PointedDripstoneDripWater,
    PointedDripstoneDripLava,
    CaveVinesPickBerries,
    BigDripleafTiltDown,
    BigDripleafTiltUp,
    CopperWaxOn,
    CopperWaxOff,
    Scrape,
    PlayerHurtDrown,
    PlayerHurtOnFire,
    PlayerHurtFreeze,
    UseSpyglass,
    StopUsingSpyglass,
    AmethystBlockChime,
    AmbientScreamer,
    HurtScreamer,
    DeathScreamer,
    MilkScreamer,
    JumpToBlock,
    PreRam,
    PreRamScreamer,
    RamImpact,
    RamImpactScreamer,
    SquidInkSquirt,
    GlowSquidInkSquirt,
    ConvertToStray,
    CakeAddCandle,
    ExtinguishCandle,
    AmbientCandle,
    BlockClick,
    BlockClickFail,
    SculkCatalystBloom,
    SculkShriekerShriek,
    WardenNearbyClose,
    WardenNearbyCloser,
    WardenNearbyClosest,
    WardenSlightlyAngry,
    RecordOtherside,
    Tongue,
    CrackIronGolem,
    RepairIronGolem,
    Listening,
    Heartbeat,
    HornBreak,
    SculkSpread = 379,
    SculkCharge = 380,
    SculkSensorPlace = 381,
    SculkShriekerPlace = 382,
    GoatCall0 = 383,
    GoatCall1 = 384,
    GoatCall2 = 385,
    GoatCall3 = 386,
    GoatCall4 = 387,
    GoatCall5 = 388,
    GoatCall6 = 389,
    GoatCall7 = 390,
    ImitateWarden = 426,
    ListeningAngry = 427,
    ItemGiven = 428,
    ItemTaken = 429,
    Disappeared = 430,
    Reappeared = 431,
    DrinkMilk = 432,
    FrogspawnHatched = 433,
    LaySpawn = 434,
    FrogspawnBreak = 435,
    SonicBoom = 436,
    SonicCharge = 437,
    ItemThrown = 438,
    Record5 = 439,
    ConvertToFrog = 440,
    RecordPlaying = 441,
    EnchantingTableUse = 442,
    StepSand = 443,
    DashReady = 444,
    BundleDropContents = 445,
    BundleInsert = 446,
    BundleRemoveOne = 447,
    PressurePlateClickOff = 448,
    PressurePlateClickOn = 449,
    ButtonClickOff = 450,
    ButtonClickOn = 451,
    DoorOpen = 452,
    DoorClose = 453,
    TrapdoorOpen = 454,
    TrapdoorClose = 455,
    FenceGateOpen = 456,
    FenceGateClose = 457,
    Insert,
    Pickup,
    InsertEnchanted,
    PickupEnchanted,
    Brush,
    BrushCompleted,
    ShatterDecoratedPot,
    BreakDecoratedPot,
    SnifferEggCrack,
    SnifferEggHatched,
    WaxedSignInteractFail,
    RecordRelic,
}
