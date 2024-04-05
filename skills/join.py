# Example usage:
# python join_markdown.py --docs chapter1 chapter2 chapter3 --output combined.md
#
import os
import argparse

def join_markdown_files(prompt_list, output_file="prompt.md"):
    output_dir = os.path.dirname(output_file)
    # Ensure the output directory exists
    if not os.path.exists(output_dir) and output_dir != '':
        os.makedirs(output_dir)

    # Open the output file in write mode
    with open(output_file, 'w') as outfile:
        # Iterate over each prompt in the list
        for prompt in prompt_list:
            # Construct the file path to the markdown file
            markdown_file_path = f"src/context/{prompt}.md"

            try:
                # Open and read the markdown file
                with open(markdown_file_path, 'r') as infile:
                    # Write the contents of the markdown file to the output file
                    outfile.write(infile.read() + "\n\n")
            except FileNotFoundError:
                print(f"File {markdown_file_path} not found.")

    print(f"Markdown files have been joined into {output_file}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Join markdown files into a single file.')
    parser.add_argument('--docs', nargs='+', required=True, help='List of markdown files to join without the .md extension')
    parser.add_argument('--output', default='prompt.md', help='Output markdown file name')
    args = parser.parse_args()

    join_markdown_files(args.docs, args.output)
