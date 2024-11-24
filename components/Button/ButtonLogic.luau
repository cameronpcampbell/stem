export type ButtonChild = Instance | string | { Instance | string }

local function StringToTextLabel(str: string)
    local TextLabel = Instance.new("TextLabel")
    TextLabel.Text = str
    return TextLabel
end

local function ParseChildToInstance(child: Instance | string, typeofChild: string)
    if typeofChild == "Instance" and ((child :: any) :: Instance).ClassName == "ImageLabel" then
        return child
    end

    if typeofChild == "string" then
        return StringToTextLabel(child :: string)
    end

    return StringToTextLabel("Button")
end

local function ParseChildrenToInstances(children: Instance | string | { Instance | string }): { Instance }
    local typeofChildren = typeof(children)


    if typeofChildren == "table" then
        local parsedChildren = table.create(#(children :: { Instance | string }))

        for idx, value in (children :: { Instance | string }) do
            parsedChildren[idx] = ParseChildToInstance(value :: ImageLabel | string, typeof(value))
        end

        return parsedChildren

    else
        return ParseChildToInstance(children :: ImageLabel | string, typeofChildren)
    end
end


return {
    ParseChildrenToInstances = ParseChildrenToInstances
}