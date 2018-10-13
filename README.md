Simple website to browse notes / watch deep dive episodes.

Current website location: http://notes.frde.me/search

Feel free to make pull requests for notes / functionality

## How to contribute timestamps / notes

- Note data is stored in /data/seasons. 

- Each json file is a topic / season.

- In the json file, follow the existing structure and add / modify notes.
  - Timestamps must be kept in sorted order to make things easy to find / modify
  - The timestamp should ideally start about a second before the topic of interest is talked about.
  - When looking at the episode on notes.frde.me/season/{id}/episode/{id}, the timestamp is present at the top right. Clicking on that timestamp will copy it to the clipboard.
  - If the website is running locally, it will auto-reload when new data is written. 
  
- Make a pull request

--------------------------------

Episodes will eventually be split into their own files. 
