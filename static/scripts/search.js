function getJson(url, callback) {
    var xhr = new XMLHttpRequest();
    xhr.open('GET', url, true);
    xhr.responseType = 'json';
    xhr.onload = function() {
        var status = xhr.status;
        if (status === 200) {
            callback(null, xhr.response);
        } else {
            callback(status, xhr.response);
        }
    };
    xhr.send();
}

let last_query = undefined;

function escapeRegExp(string) {
  return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'); // $& means the whole matched string
}

function search() {
    let search_text = document.getElementById("search_text");

    let query = escapeRegExp(search_text.value);
    console.log(query);

    if(last_query === query){
        return;
    }

    last_query = query;

    window.location.hash = query;

    getJson("/search/json?q="+encodeURIComponent(query), function(status, result) {
        if(status !== null) {
            console.log("Could not search, status code: "+status + " - " + result);
            return;
        }

        let reg_query = new RegExp(last_query, "ig");

        let results_node = document.getElementById("results");

        let seasons_list = document.createElement("ul");
        seasons_list.id = "seasons";

        for(let season_index in result) {
            let season = result[season_index];

            let season_results_node = document.createElement("li");
            season_results_node.className = "season";
            seasons_list.appendChild(season_results_node);

            // Season title
            let season_title_node = document.createElement("h1");
            season_title_node.innerText = season.title;
            season_results_node.appendChild(season_title_node);

            // Episodes
            let episodes_list = document.createElement("ul");
            episodes_list.id = "episodes";
            season_results_node.appendChild(episodes_list);

            for(let episode_index in season.episode_results) {
                let episode = season.episode_results[episode_index];

                let episode_results_node = document.createElement("li");
                episode_results_node.className = "episode";
                episodes_list.appendChild(episode_results_node);

                // Episode title
                let episode_title = document.createElement("h3");
                episode_title.innerText = episode.title;
                episode_results_node.appendChild(episode_title);

                let note_list = document.createElement("ul");
                note_list.id = "notes";
                episode_results_node.appendChild(note_list);

                for(let note_index in episode.note_results) {
                    let note = episode.note_results[note_index];

                    let note_result = document.createElement("li");
                    note_result.className = "note";
                    note_list.appendChild(note_result);

                    let note_link = document.createElement("a");
                    note_link.href = "/season/"+season.id+"/episode/"+episode.id+"?t="+note.timestamp;

                    let description = note.description.replace(reg_query, "<span class='highlight'>"+query+"</span>");

                    note_link.innerHTML = description;
                    note_result.appendChild(note_link);
                }
            }
        }

        results_node.innerHTML = "";
        results_node.appendChild(seasons_list);
    });
}

window.onload = function() {
    let search_text = document.getElementById("search_text");
    document.getElementById("search_text").addEventListener("input", search)
    if(window.location.hash) {
        search_text.value = window.location.hash.substring(1);
    }
    search();
};