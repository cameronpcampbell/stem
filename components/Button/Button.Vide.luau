--!strict


--> Modules -------------------------------------------------------------------------------------------
local Packages = script.Parent.Parent.Parent.Packages
local Vide = require(Packages._Index["centau_vide@0.3.1"].vide)

local Logic = require(script.Parent.ButtonLogic)
local Styles = script.Parent.ButtonStyles
-------------------------------------------------------------------------------------------------------


--> Types ---------------------------------------------------------------------------------------------
type ButtonProps = {
    Children: Logic.ButtonChild
}
-------------------------------------------------------------------------------------------------------


--> Variables -----------------------------------------------------------------------------------------
local Create = Vide.create

local ParseChildrenToInstances = Logic.ParseChildrenToInstances
-------------------------------------------------------------------------------------------------------


return function(props: ButtonProps)
    local children = ParseChildrenToInstances(props.Children)

    return Create "ImageButton" {
        children,

        Create "StyleLink" { StyleSheet = Styles }
    }
end