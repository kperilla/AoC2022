MODFILE = mod.rs
MODFILE_CONTENTS = pub mod $(PACKAGE_NAME);
CODE_STARTER = pub fn part1(input: String) {}\npub fn part2(input: String) {}

testy:
	echo "$(CODE_STARTER)"

package:
	mkdir src/$(PACKAGE_NAME)
	echo "$(MODFILE_CONTENTS)" > src/$(PACKAGE_NAME)/$(MODFILE)
	echo "$(CODE_STARTER)" > src/$(PACKAGE_NAME)/$(PACKAGE_NAME).rs

