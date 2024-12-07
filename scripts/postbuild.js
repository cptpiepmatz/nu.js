import fs from "fs";
import path from "path";

const baseDir = "pkg";

// Find all .gitignore files in pkg/*/
fs.readdirSync(baseDir, { withFileTypes: true })
  .filter((entry) => entry.isDirectory()) // Only look at directories
  .forEach((dir) => {
    const gitignorePath = path.join(baseDir, dir.name, ".gitignore");

    if (fs.existsSync(gitignorePath)) {
      try {
        fs.unlinkSync(gitignorePath);
        console.log(`Deleted: ${gitignorePath}`);
      } catch (err) {
        console.error(`Failed to delete ${gitignorePath}: ${err.message}`);
      }
    }
  });
