# Recipe Converter

This project is a continuation of the project found under this link: 
https://github.com/ChrisEwert/cookbook

The latter, written in Java, offers an interactive console where users can create new recipes. These recipes are then saved in a JSON file.

In contrast, this Rust project allows users to specify a link to a recipe JSON file. The content of the JSON file is then extracted and transformed into an MD file and an ADOC file.

This results in 2 nicely organized cookbooks that are stored in a separate "results" directory.

If you do not want to use the first project, you can use the example data, which you can find under "example_data/recipes.json". 
The resulting cookbook files should correspond to those in "expected_results/".
