use minifb::Scale;

#[derive(Debug)]
pub struct Pattern {
    pub name: &'static str,
    pub width: usize,
    pub height: usize,
    pub scale: Scale,
    pub rle_string: &'static str,
}

pub fn get_patterns() -> Vec<Pattern> {
    vec![
        Pattern {
            name: "Triangle",
            width: 2,
            height: 2,
            scale: Scale::X4,
            rle_string: "oo$oo!",
        },
        Pattern {
            name: "Glider",
            width: 3,
            height: 3,
            scale: Scale::X4,
            rle_string: "bo$2bo$3o!",
        },
        Pattern {
            name: "Gosper Glider Gun",
            width: 36,
            height: 9,
            scale: Scale::X4,
            rle_string: "24bo11b$22bobo11b$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o14b$2o8b
            o3bob2o4bobo11b$10bo5bo7bo11b$11bo3bo20b$12b2o!",
        },
        Pattern {
            name: "Quasar3",
            width: 61,
            height: 61,
            scale: Scale::X4,
            rle_string: "26b3o3b3o2$24bo4bobo4bo$24bo4bobo4bo$24bo4bobo4bo$26b3o3b3o2$24b3o7b3o
            $18b3o2bo4bo3bo4bo2b3o$23bo4bo3bo4bo$16bo4bobo4bo3bo4bobo4bo$16bo4bo
            17bo4bo$16bo4bo2b3o7b3o2bo4bo$18b3o19b3o2$16b3o23b3o$10b3o2bo4bo19bo4b
            o2b3o$15bo4bo19bo4bo$8bo4bobo4bo19bo4bobo4bo$8bo4bo33bo4bo$8bo4bo2b3o
            23b3o2bo4bo$10b3o35b3o2$8b3o39b3o$2b3o2bo4bo35bo4bo2b3o$7bo4bo35bo4bo$
            o4bobo4bo35bo4bobo4bo$o4bo49bo4bo$o4bo2b3o39b3o2bo4bo$2b3o51b3o2$2b3o
            51b3o$o4bo2b3o39b3o2bo4bo$o4bo49bo4bo$o4bobo4bo35bo4bobo4bo$7bo4bo35bo
            4bo$2b3o2bo4bo35bo4bo2b3o$8b3o39b3o2$10b3o35b3o$8bo4bo2b3o23b3o2bo4bo$
            8bo4bo33bo4bo$8bo4bobo4bo19bo4bobo4bo$15bo4bo19bo4bo$10b3o2bo4bo19bo4b
            o2b3o$16b3o23b3o2$18b3o19b3o$16bo4bo2b3o7b3o2bo4bo$16bo4bo17bo4bo$16bo
            4bobo4bo3bo4bobo4bo$23bo4bo3bo4bo$18b3o2bo4bo3bo4bo2b3o$24b3o7b3o2$26b
            3o3b3o$24bo4bobo4bo$24bo4bobo4bo$24bo4bobo4bo2$26b3o3b3o!",
        },
        Pattern {
            name: "Backward Space Rake",
            width: 23,
            height: 20,
            scale: Scale::X4,
            rle_string: "12b2o5b4o$10b2ob2o3bo3bo$10b4o8bo$11b2o5bo2bob2$9bo13b$8b2o8b2o3b$7bo
            9bo2bo2b$8b5o4bo2bo2b$9b4o3b2ob2o2b$12bo4b2o4b4$19b4o$18bo3bo$b4o17bo$
            o3bo13bo2bob$4bo18b$o2bo!",
        },
        Pattern {
            name: "Pi ship 1",
            width: 99,
            height: 29,
            scale: Scale::X4,
            rle_string: "7bo83bo$6b3o81b3o$4b2ob3o20b3o9b3o9b3o9b3o20b3ob2o$5bo2bob2o4bo4bo7bo
            3bo7bo3bo7bo3bo7bo3bo7bo4bo4b2obo2bo$2b2obo4bobob2ob2ob3o5b2o3b2o5b2o
            3b2o5b2o3b2o5b2o3b2o5b3ob2ob2obobo4bob2o$2b2obobo2bobo7b4o3b2obobob2o
            3b2obobob2o3b2obobob2o3b2obobob2o3b4o7bobo2bobob2o$2bo8b3obobob2o2b2ob
            2ob2ob2ob2ob2ob2ob2ob2ob2ob2ob2ob2ob2ob2ob2ob2ob2o2b2obobob3o8bo$b2o7b
            2o12bobo3bobo3bobo3bobo3bobo3bobo3bobo3bobo3bobo12b2o7b2o2$5b3o15b2ob
            2ob2ob2ob2ob2ob2ob2ob2ob2ob2ob2ob2ob2ob2ob2ob2ob2o15b3o$4bo3bo81bo3bo$
            3b2o4bo11b57o11bo4b2o$2bobob2ob2o3b3o2b2obo4bo5bo8bo6bo6bo8bo5bo4bob2o
            2b3o3b2ob2obobo$b2obo4bob2ob3obo61bob3ob2obo4bob2o$o4bo3bo4bobo4bo4bob
            obobobo6bobob2obobob2obobo6bobobobobo4bo4bobo4bo3bo4bo$12bo5b2o16bo2bo
            19bo2bo16b2o5bo$2o7b2o25bo2bo19bo2bo25b2o7b2o2$36bo2bo19bo2bo$37b2o21b
            2o5$49bo$48b3o$47bo3bo$37b2o8b2ob2o8b2o$37b2o21b2o!",
        },
        Pattern {
            name: "Max",
            width: 27,
            height: 27,
            scale: Scale::X4,
            rle_string: "18bo8b$17b3o7b$12b3o4b2o6b$11bo2b3o2bob2o4b$10bo3bobo2bobo5b$10bo4bobo
            bobob2o2b$12bo4bobo3b2o2b$4o5bobo4bo3bob3o2b$o3b2obob3ob2o9b2ob$o5b2o
            5bo13b$bo2b2obo2bo2bob2o10b$7bobobobobobo5b4o$bo2b2obo2bo2bo2b2obob2o
            3bo$o5b2o3bobobo3b2o5bo$o3b2obob2o2bo2bo2bob2o2bob$4o5bobobobobobo7b$
            10b2obo2bo2bob2o2bob$13bo5b2o5bo$b2o9b2ob3obob2o3bo$2b3obo3bo4bobo5b4o
            $2b2o3bobo4bo12b$2b2obobobobo4bo10b$5bobo2bobo3bo10b$4b2obo2b3o2bo11b$
            6b2o4b3o12b$7b3o17b$8bo!",
        },
    ]
}
