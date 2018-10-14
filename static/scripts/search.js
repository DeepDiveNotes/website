Number.prototype.toHHMMSS = function () {
    var sec_num = parseInt(this, 10); // don't forget the second param
    var hours   = Math.floor(sec_num / 3600);
    var minutes = Math.floor((sec_num - (hours * 3600)) / 60);
    var seconds = sec_num - (hours * 3600) - (minutes * 60);

    if (hours   < 10) {hours   = "0"+hours;}
    if (minutes < 10) {minutes = "0"+minutes;}
    if (seconds < 10) {seconds = "0"+seconds;}
    return hours+':'+minutes+':'+seconds;
};

const search_text = document.getElementById("search_text");

const e = React.createElement;

class SearchResults extends React.Component {
    constructor(props) {
        super(props);

        this.search = this.search.bind(this);
        this.query = this.query.bind(this);

        this.state = {
            results: null,
            query: this.query(search_text.value)
        };

        search_text.addEventListener("input", this.search);
    }

    search() {
        fetch("/search/json?q="+search_text.value)
            .then(response => response.json())
            .then(results => {
                this.setState({
                    results,
                    query: this.query(search_text.value)
                })
            });
    }

    query(value) {
        if(value.length > 0) {
            return new RegExp('('+value+')', "ig");
        }

        return null;
    }

    componentDidMount() {
        this.search();
    }


    render() {
        if(this.state.results != null){
            return (
                <SeasonList query = {this.state.query} seasons={this.state.results} />
            );
        } else {
            return (
                <div>LOADING</div>
            )
        }
    }
}

class SeasonList extends React.Component {
    render() {
        let seasons = this.props.seasons.map(season => {
            return (
                <li class="season">
                    <h1>{season.title}</h1>
                    <EpisodeList query={this.props.query} season_id={season.id} episodes={season.episode_results}/>
                </li>
            );
        });

        return (
            <ul id="seasons">{seasons}</ul>
        );
    }
}

class EpisodeList extends React.Component {
    render() {
        let episodes = this.props.episodes.map(episode => {
            return(<li class="episode">
                <h2>{episode.title}</h2>
                <NoteList query = {this.props.query} season_id={this.props.season_id} episode_id={episode.id} notes={episode.note_results}/>
            </li>);
        });

        return(<ul id="episodes">{episodes}</ul>)
    }
}

class NoteList extends React.Component {
    render() {
        let notes = this.props.notes.map(note => {

            let description = note.description;

            if(this.props.query !== null) {
                description = description.replace(this.props.query, "<span class='highlight'>$1</span>")
            }

            return(<li class="note">
                <a href={"/season/"+this.props.season_id + "/episode/"+this.props.episode_id+"?t="+note.timestamp}><span class="timestamp">{note.timestamp.toHHMMSS()}</span> - <span dangerouslySetInnerHTML={{__html: description}} /></a>
            </li>)
        });

        return(<ul id="notes">{notes}</ul>);
    }
}

if(window.location.hash) {
    search_text.value = window.location.hash.substring(1);
}

const results_container = document.querySelector("#search_results_container");
ReactDOM.render(e(SearchResults), results_container);






