-- Strategy: 7road ddtank
-- Comment: You need to finish real-name authentication first!

function login(username, password, server_id)
    local agent = agent()

    -- Get server url
    local index_page = agent:get("https://ddt.wan.com/server/index.html")
    local game_url = ""

    for url, server_id_in_page in string.gmatch(index_page, [[<li.-class.-["'].-die10.->.-href=.-["'](.-)["'].->s(%d+).-<]]) do
        game_url = url
        if server_id_in_page == server_id then
            break
        end
    end

    -- Login www.wan.com
    agent:get("http://www.wan.com/accounts/checklogin.html?cn="
        .. username .. "&pwd=" .. string.lower(crypto.md5(password)));

    local response, assist_url_head = agent:get_with(game_url)
    local assist_url_path = string.match(response, [[<iframe.-id=.-url_mainframe.-src=["']/(.-)["'].->]])

    local assist_url = assist_url_head .. assist_url_path

    response, assist_url_head = agent:get_with(assist_url)
    assist_url = string.match(response, [[<iframe.-id=.-url_mainframe.-src=["'](.-)["'].->]])

    local assist_doc, url = agent:get_with(assist_url)
    local params = string.match(assist_doc, [[<param.-name.-movie.-value=['"](.-)['"] />]])

    return url .. params
end
