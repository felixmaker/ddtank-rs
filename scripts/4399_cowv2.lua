function login(username, password, server_id)
    local cookie_cowv2 = get_cookie_by_cowv2("http://web.4399.com/user/?_a=login&redirecturl=%2Fuser%2Fuserinfo.php",
        "Uauth", "http://web.4399.com/user/?_a=login&redirecturl=%2Fuser%2Fuserinfo.php")
    local cookie1 = string.match(cookie_cowv2, [[#"(.-)"#]])
    local cookie, _ = string.gsub(cookie1, [[\]], [[]])

    local agent = agent()
    agent:load_cookie("http://web.4399.com/", cookie)
    local response = agent:get("http://web.4399.com/stat/togame.php?target=ddt&server_id=S" .. server_id)
    local assist_page = string.match(response, [[iframe.-game_box.-src="(.-)"]])
    local gamepage, url = agent:get_with(assist_page)
    local params = string.match(gamepage, [[<param.-name.-movie.-value=['"](.-)['"] />]])
    return url .. params
end
