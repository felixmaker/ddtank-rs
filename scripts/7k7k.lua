-- Strategy: 7k7k ddtank

function login(username, password, server_id)
    local agent = agent()
    local k7_form = {
        ["username"] = username,
        ["password"] = password,
        ["auto"] = "1",
        ["formtype"] = "index_log"
    }

    agent:post("http://web.7k7k.com/source/Post.php", k7_form)
    local text = agent:get("http://web.7k7k.com/games/togame.php?target=ddt_7&server_id=" .. server_id)
    local assist_url = string.match(text, [[input.-url.-value="(.-)"]]);
    local assist_doc, url = agent:get_with(assist_url)
    local params = string.match(assist_doc, [[<param.-name.-movie.-value=['"](.-)['"] />]])

    local game_url = url .. params

    return game_url
end
