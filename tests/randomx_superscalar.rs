extern crate mithril;

use mithril::randomx::superscalar::{ScProgram};

#[test]
fn test_generate() {
    //let prog = ScProgram::generate();
}
/*
	runTest("SuperscalarHash generator", RANDOMX_SUPERSCALAR_LATENCY == 170, []() {
		char sprogHash[32];
		randomx::SuperscalarProgram sprog;
		const char key[] = "test key 000";
		constexpr size_t keySize = sizeof(key) - 1;
		randomx::Blake2Generator gen(key, keySize);

		const char superscalarReferences[10][65] = {
			"d3a4a6623738756f77e6104469102f082eff2a3e60be7ad696285ef7dfc72a61",
			"f5e7e0bbc7e93c609003d6359208688070afb4a77165a552ff7be63b38dfbc86",
			"85ed8b11734de5b3e9836641413a8f36e99e89694f419c8cd25c3f3f16c40c5a",
			"5dd956292cf5d5704ad99e362d70098b2777b2a1730520be52f772ca48cd3bc0",
			"6f14018ca7d519e9b48d91af094c0f2d7e12e93af0228782671a8640092af9e5",
			"134be097c92e2c45a92f23208cacd89e4ce51f1009a0b900dbe83b38de11d791",
			"268f9392c20c6e31371a5131f82bd7713d3910075f2f0468baafaa1abd2f3187",
			"c668a05fd909714ed4a91e8d96d67b17e44329e88bc71e0672b529a3fc16be47",
			"99739351315840963011e4c5d8e90ad0bfed3facdcb713fe8f7138fbf01c4c94",
			"14ab53d61880471f66e80183968d97effd5492b406876060e595fcf9682f9295",
		};

		for (int i = 0; i < 10; ++i) {
			randomx::generateSuperscalar(sprog, gen);
			blake2b(sprogHash, sizeof(sprogHash), &sprog.programBuffer, sizeof(randomx::Instruction) * sprog.getSize(), nullptr, 0);
			assert(equalsHex(sprogHash, superscalarReferences[i]));
		}
	});
*/



const EXPECTED_SUPERSCALAR_PROG : &str = r#"IADD_RS r0, r6, SHFT 0
IADD_RS r1, r2, SHFT 0
IADD_RS r4, r3, SHFT 0
IADD_RS r6, r2, SHFT 1
IADD_RS r6, r7, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r2, r7, SHFT 0
IADD_RS r3, r5, SHFT 0
IADD_RS r7, r7, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r6, r2, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r5, r0, 0, SHFT 0
IADD_RS r2, r1, SHFT 0
IADD_RS r7, r1, SHFT 3
IADD_RS r1, r1, SHFT 0
IADD_RS r3, r7, SHFT 0
IADD_RS r1, r0, SHFT 0
IADD_RS r0, r7, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r3, r1, SHFT 0
IADD_RS r4, r0, SHFT 0
IADD_RS r6, r7, SHFT 0
IADD_RS r7, r7, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r5, r5, 60, SHFT 0
IADD_RS r7, r1, SHFT 0
IADD_RS r0, r3, SHFT 0
IADD_RS r2, r1, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r1, r5, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r5, r3, 0, SHFT 0
IADD_RS r3, r6, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r5, r7, 0, SHFT 0
IADD_RS r4, r7, SHFT 0
IADD_RS r6, r5, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r5, r1, 0, SHFT 0
IADD_RS r4, r1, SHFT 0
IADD_RS r0, r1, SHFT 3
IADD_RS r2, r0, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r1, r7, SHFT 0
IADD_RS r7, r0, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r7, r0, SHFT 0
IADD_RS r5, r2, 0, SHFT 0
IADD_RS r2, r0, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r1, r5, SHFT 0
IADD_RS r6, r0, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r3, r7, SHFT 2
IADD_RS r5, r5, 374267952, SHFT 0
IADD_RS r3, r0, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r7, r5, SHFT 0
IADD_RS r5, r0, 0, SHFT 0
IADD_RS r2, r7, SHFT 0
IADD_RS r7, r0, SHFT 0
IADD_RS r5, r0, 0, SHFT 0
IADD_RS r3, r2, SHFT 0
IADD_RS r1, r4, SHFT 3
IADD_RS r4, r2, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r4, r1, SHFT 0
IADD_RS r4, r6, SHFT 0
IADD_RS r6, r6, SHFT 0
IADD_RS r0, r1, SHFT 0
IADD_RS r6, r4, SHFT 0
IADD_RS r5, r1, 0, SHFT 0
IADD_RS r4, r0, SHFT 0
IADD_RS r1, r2, SHFT 0
IADD_RS r6, r0, SHFT 0
IADD_RS r5, r5, 12, SHFT 0
IADD_RS r7, r3, SHFT 2
IADD_RS r0, r0, SHFT 0
IADD_RS r2, r5, SHFT 0
IADD_RS r7, r7, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r1, r0, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r6, r4, SHFT 0
IADD_RS r4, r1, SHFT 0
IADD_RS r4, r5, SHFT 0
IADD_RS r0, r2, SHFT 0
IADD_RS r2, r3, SHFT 0
IADD_RS r6, r1, SHFT 1
IADD_RS r3, r3, SHFT 0
IADD_RS r3, r5, SHFT 0
IADD_RS r5, r1, 0, SHFT 0
IADD_RS r1, r5, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r5, r5, -122520312, SHFT 0
IADD_RS r4, r7, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r6, r6, SHFT 0
IADD_RS r4, r5, SHFT 0
IADD_RS r6, r5, SHFT 0
IADD_RS r5, r4, 0, SHFT 0
IADD_RS r1, r4, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r0, r4, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r0, r4, SHFT 0
IADD_RS r7, r0, SHFT 0
IADD_RS r6, r6, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r2, r3, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r0, r5, SHFT 0
IADD_RS r5, r5, -1103157738, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r6, r1, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r7, r1, SHFT 0
IADD_RS r1, r4, SHFT 0
IADD_RS r3, r6, SHFT 0
IADD_RS r4, r6, SHFT 0
IADD_RS r7, r6, SHFT 0
IADD_RS r1, r3, SHFT 0
IADD_RS r6, r6, SHFT 0
IADD_RS r2, r3, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r5, r3, 0, SHFT 0
IADD_RS r0, r3, SHFT 3
IADD_RS r3, r3, SHFT 0
IADD_RS r0, r6, SHFT 0
IADD_RS r5, r6, 0, SHFT 0
IADD_RS r3, r7, SHFT 0
IADD_RS r2, r4, SHFT 0
IADD_RS r6, r0, SHFT 1
IADD_RS r0, r0, SHFT 0
IADD_RS r6, r1, SHFT 0
IADD_RS r4, r7, SHFT 0
IADD_RS r7, r4, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r5, r6, 0, SHFT 0
IADD_RS r5, r5, 28, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r1, r4, SHFT 0
IADD_RS r6, r5, SHFT 0
IADD_RS r4, r1, SHFT 0
IADD_RS r7, r2, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r2, r5, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r3, r5, SHFT 0
IADD_RS r3, r2, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r6, r1, SHFT 0
IADD_RS r3, r5, SHFT 0
IADD_RS r5, r0, 0, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r2, r6, SHFT 0
IADD_RS r3, r1, SHFT 0
IADD_RS r0, r1, SHFT 3
IADD_RS r7, r7, SHFT 0
IADD_RS r6, r1, SHFT 0
IADD_RS r1, r0, SHFT 0
IADD_RS r6, r0, SHFT 0
IADD_RS r6, r0, SHFT 3
IADD_RS r1, r1, SHFT 0
IADD_RS r7, r6, SHFT 3
IADD_RS r5, r0, 0, SHFT 0
IADD_RS r1, r2, SHFT 0
IADD_RS r4, r6, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r6, r7, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r7, r0, SHFT 0
IADD_RS r0, r7, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r5, r5, 1612707140, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r6, r1, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r3, r7, SHFT 0
IADD_RS r7, r3, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r5, r2, 0, SHFT 0
IADD_RS r2, r3, SHFT 0
IADD_RS r6, r3, SHFT 1
IADD_RS r1, r1, SHFT 0
IADD_RS r4, r3, SHFT 0
IADD_RS r3, r6, SHFT 0
IADD_RS r6, r1, SHFT 0
IADD_RS r1, r4, SHFT 3
IADD_RS r3, r3, SHFT 0
IADD_RS r7, r0, SHFT 3
IADD_RS r1, r4, SHFT 0
IADD_RS r7, r3, SHFT 0
IADD_RS r6, r2, SHFT 0
IADD_RS r0, r5, SHFT 3
IADD_RS r3, r4, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r4, r2, SHFT 0
IADD_RS r5, r0, 0, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r0, r5, SHFT 0
IADD_RS r3, r2, SHFT 0
IADD_RS r5, r7, 0, SHFT 0
IADD_RS r4, r1, SHFT 0
IADD_RS r7, r2, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r7, r2, SHFT 0
IADD_RS r1, r6, SHFT 0
IADD_RS r1, r0, SHFT 0
IADD_RS r2, r7, SHFT 0
IADD_RS r7, r7, SHFT 0
IADD_RS r6, r2, SHFT 0
IADD_RS r0, r6, SHFT 0
IADD_RS r5, r5, 878756540, SHFT 0
IADD_RS r7, r6, SHFT 0
IADD_RS r2, r3, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r6, r6, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r3, r5, SHFT 0
IADD_RS r6, r6, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r1, r0, SHFT 0
IADD_RS r7, r1, SHFT 0
IADD_RS r5, r5, 0, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r7, r0, SHFT 0
IADD_RS r3, r6, SHFT 0
IADD_RS r1, r4, SHFT 1
IADD_RS r0, r4, SHFT 3
IADD_RS r4, r4, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r6, r4, SHFT 3
IADD_RS r0, r0, SHFT 0
IADD_RS r6, r1, SHFT 0
IADD_RS r1, r2, SHFT 0
IADD_RS r6, r2, SHFT 0
IADD_RS r4, r3, SHFT 0
IADD_RS r0, r2, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r5, r0, 0, SHFT 0
IADD_RS r2, r0, SHFT 0
IADD_RS r0, r3, SHFT 0
IADD_RS r7, r7, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r2, r1, SHFT 2
IADD_RS r3, r3, SHFT 0
IADD_RS r2, r5, SHFT 0
IADD_RS r2, r4, SHFT 0
IADD_RS r5, r5, 0, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r0, r3, SHFT 0
IADD_RS r3, r4, SHFT 0
IADD_RS r1, r4, SHFT 3
IADD_RS r7, r7, SHFT 0
IADD_RS r6, r6, SHFT 0
IADD_RS r7, r1, SHFT 2
IADD_RS r6, r4, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r3, r6, SHFT 0
IADD_RS r1, r6, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r5, r7, 0, SHFT 0
IADD_RS r7, r6, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r2, r4, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r0, r6, SHFT 0
IADD_RS r4, r2, SHFT 0
IADD_RS r2, r0, SHFT 0
IADD_RS r6, r6, SHFT 0
IADD_RS r0, r6, SHFT 0
IADD_RS r4, r3, SHFT 0
IADD_RS r6, r6, SHFT 0
IADD_RS r2, r3, SHFT 0
IADD_RS r1, r0, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r5, r5, 49, SHFT 0
IADD_RS r7, r7, SHFT 0
IADD_RS r3, r7, SHFT 0
IADD_RS r0, r5, SHFT 0
IADD_RS r5, r5, 917412870, SHFT 0
IADD_RS r3, r0, SHFT 0
IADD_RS r7, r6, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r0, r4, SHFT 0
IADD_RS r5, r1, 0, SHFT 0
IADD_RS r6, r4, SHFT 0
IADD_RS r6, r6, SHFT 0
IADD_RS r2, r4, SHFT 0
IADD_RS r6, r2, SHFT 0
IADD_RS r1, r4, SHFT 0
IADD_RS r6, r6, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r1, r6, SHFT 0
IADD_RS r7, r4, SHFT 0
IADD_RS r6, r4, SHFT 0
IADD_RS r1, r2, SHFT 0
IADD_RS r4, r3, SHFT 1
IADD_RS r0, r0, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r2, r0, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r5, r5, 55, SHFT 0
IADD_RS r6, r7, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r7, r1, SHFT 0
IADD_RS r3, r1, SHFT 0
IADD_RS r5, r0, 0, SHFT 0
IADD_RS r3, r6, SHFT 0
IADD_RS r6, r4, SHFT 0
IADD_RS r7, r7, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r1, r4, SHFT 2
IADD_RS r7, r1, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r5, r2, 0, SHFT 0
IADD_RS r4, r2, SHFT 0
IADD_RS r5, r5, -502592121, SHFT 0
IADD_RS r0, r1, SHFT 0
IADD_RS r1, r2, SHFT 0
IADD_RS r7, r7, SHFT 0
IADD_RS r6, r2, SHFT 0
IADD_RS r6, r6, SHFT 0
IADD_RS r7, r2, SHFT 0
IADD_RS r3, r4, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r7, r7, SHFT 0
IADD_RS r2, r4, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r5, r5, -50779412, SHFT 0
IADD_RS r7, r6, SHFT 2
IADD_RS r5, r7, 0, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r5, r6, 0, SHFT 0
IADD_RS r3, r1, SHFT 0
IADD_RS r7, r6, SHFT 0
IADD_RS r5, r3, 0, SHFT 0
IADD_RS r3, r2, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r6, r0, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r4, r2, SHFT 0
IADD_RS r0, r6, SHFT 0
IADD_RS r7, r0, SHFT 2
IADD_RS r0, r0, SHFT 0
IADD_RS r7, r7, SHFT 0
IADD_RS r2, r1, SHFT 0
IADD_RS r1, r5, SHFT 0
IADD_RS r7, r6, SHFT 0
IADD_RS r4, r4, SHFT 0
IADD_RS r0, r6, SHFT 1
IADD_RS r3, r3, SHFT 0
IADD_RS r6, r4, SHFT 0
IADD_RS r5, r5, 1805329716, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r2, r1, SHFT 1
IADD_RS r4, r7, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r1, r0, SHFT 0
IADD_RS r0, r7, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r2, r7, SHFT 0
IADD_RS r4, r5, SHFT 0
IADD_RS r7, r3, SHFT 1
IADD_RS r6, r6, SHFT 0
IADD_RS r6, r3, SHFT 0
IADD_RS r5, r3, 0, SHFT 0
IADD_RS r3, r1, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r7, r7, SHFT 0
IADD_RS r6, r6, SHFT 0
IADD_RS r5, r5, -893676238, SHFT 0
IADD_RS r6, r1, SHFT 0
IADD_RS r1, r5, SHFT 0
IADD_RS r0, r4, SHFT 0
IADD_RS r5, r5, 233018490, SHFT 0
IADD_RS r7, r4, SHFT 0
IADD_RS r6, r7, SHFT 0
IADD_RS r1, r4, SHFT 2
IADD_RS r1, r4, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r4, r7, SHFT 0
IADD_RS r2, r4, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r5, r7, 0, SHFT 0
IADD_RS r7, r7, SHFT 0
IADD_RS r6, r0, SHFT 0
IADD_RS r4, r7, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r3, r6, SHFT 0
IADD_RS r1, r7, SHFT 0
IADD_RS r0, r6, SHFT 3
IADD_RS r5, r0, 0, SHFT 0
IADD_RS r7, r7, SHFT 0
IADD_RS r0, r5, SHFT 0
IADD_RS r6, r7, SHFT 0
IADD_RS r6, r6, SHFT 0
IADD_RS r0, r0, SHFT 0
IADD_RS r2, r0, SHFT 0
IADD_RS r7, r6, SHFT 0
IADD_RS r2, r5, SHFT 0
IADD_RS r4, r0, SHFT 0
IADD_RS r6, r0, SHFT 3
IADD_RS r5, r5, -349722477, SHFT 0
IADD_RS r6, r3, SHFT 0
IADD_RS r1, r0, SHFT 0
IADD_RS r0, r1, SHFT 0
IADD_RS r2, r2, SHFT 0
IADD_RS r7, r7, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r6, r4, SHFT 3
IADD_RS r5, r7, 0, SHFT 0
IADD_RS r7, r6, SHFT 0
IADD_RS r2, r1, SHFT 0
IADD_RS r6, r6, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r6, r6, SHFT 0
IADD_RS r0, r4, SHFT 0
IADD_RS r4, r6, SHFT 1
IADD_RS r3, r3, SHFT 0
IADD_RS r1, r1, SHFT 0
IADD_RS r3, r3, SHFT 0
IADD_RS r0, r5, SHFT 0"#;