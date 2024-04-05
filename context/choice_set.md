Generate a set of choices based off of the following stpry context.

Return each a group of choices in this format:

~~~json
[
  {
    "label": "Report the information to the city guard",
    "consequence": "You decide to report the information to the city guard. After your shift you head to the closest Watch outpust. [...] You contemplate your fate, arrested for being an accomplice to the crime, despite doing what you thought was right and reporting what you heard. [...]"
  },
  {
    "label": "Investigate the warehouse yourself",
    "consequence": "You decide to investigate the warehouse yourself. After your shift, you head to the warehouse and find it to be a trap. [...]"
  },
  {
    "label": "Hint that you might want to join their next heist",
    "consequence": "You decide to hint that you might want to join their next heist. After your shift, you approach the group and express your interest in joining them. [...]"
  },
  {
    "label": "Ignore the information and continue with your day",
    "consequence": "You decide to ignore the information and continue with your day. After your shift, you head home and try to forget what you heard. [...]"
  }
]
~~~

Don't actually include the "[...]" in the `consequence` field. That's just a placeholder for the rest of the story. Write a paragraph or two for each choice to describe what happens next to give some instant feedback while the further part of the story will be generated later.

DON'T include less than 2 choices, or more than 5 choices
DO occasionally add twists in the consequence, like the example of being arrested when reporting the crime yourself in the example above.

Here is the story context:
