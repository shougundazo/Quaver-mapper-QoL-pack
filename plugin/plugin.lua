-- Quaver runtime entrypoint.
-- Generated from main.lua for distribution; kept in sync manually for MVP.

local DEFAULT_LABEL = "QoL bookmark"

local function get_value(key, fallback)
    local value = state.GetValue(key)
    if value == nil then
        return fallback
    end
    return value
end

local function set_value(key, value)
    state.SetValue(key, value)
end

local function current_time()
    return math.floor(state.SongTime + 0.5)
end

local function selected_count()
    local selected = state.SelectedHitObjects
    if selected == nil then
        return 0
    end
    return #selected
end

local function current_bookmark_note()
    local bookmark = state.CurrentBookmark
    if bookmark == nil then
        return ""
    end
    return bookmark.Note or ""
end

local function draw_bookmark_tools()
    local label = get_value("bookmark_label", DEFAULT_LABEL)
    imgui.Text("Current time: " .. tostring(current_time()) .. " ms")
    imgui.Text("Selected notes: " .. tostring(selected_count()))

    local existing = current_bookmark_note()
    if existing ~= "" then
        imgui.Text("Current bookmark: " .. existing)
    else
        imgui.Text("Current bookmark: none")
    end

    local changed, next_label = imgui.InputText("Label", label, 128)
    if changed then
        label = next_label
        set_value("bookmark_label", label)
    end

    if imgui.Button("Add bookmark here") then
        actions.AddBookmark(current_time(), label)
        print("S!", "Bookmark added at " .. tostring(current_time()) .. " ms")
    end

    imgui.SameLine()

    if imgui.Button("Remember request") then
        write({
            lastBookmarkRequest = {
                time = current_time(),
                label = label,
                unixTime = state.UnixTime
            }
        })
        print("I!", "Bookmark request saved to plugin config.")
    end
end

local function draw_qol_info()
    imgui.Text("Mode: " .. tostring(map.Mode))
    imgui.Text("HitObjects: " .. tostring(#map.HitObjects))
    imgui.Text("TimingPoints: " .. tostring(#map.TimingPoints))
    imgui.Text("Bookmarks: " .. tostring(#map.Bookmarks))
    imgui.Separator()
    imgui.TextWrapped("MVP external app integration is manual: launch the Tauri or CLI app and select the .qua file there.")
end

function draw()
    imgui.Begin("Quaver Mapper QoL Pack")
    draw_qol_info()
    imgui.Separator()
    draw_bookmark_tools()
    state.IsWindowHovered = imgui.IsWindowHovered()
    imgui.End()
end
