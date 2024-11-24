--!strict


--> Modules -------------------------------------------------------------------------------------------
local Packages = script.Parent.Parent.Parent.Packages
local Fusion = require(Packages._Index["elttob_fusion@0.3.0"].fusion)

local Logic = require(script.Parent.ButtonLogic)
local Styles = script.Parent.ButtonStyles
-------------------------------------------------------------------------------------------------------


--> Types ---------------------------------------------------------------------------------------------
type ButtonScope = Fusion.Scope<{ New: typeof(Fusion.New) }>

type ButtonProps = {
    Children: Logic.ButtonChild
}
-------------------------------------------------------------------------------------------------------


--> Variables -----------------------------------------------------------------------------------------
local Children = Fusion.Children

local ParseChildrenToInstances = Logic.ParseChildrenToInstances
-------------------------------------------------------------------------------------------------------


return function(scope: ButtonScope, props: ButtonProps)
    local children = ParseChildrenToInstances(props.Children) :: Fusion.Child

    return scope:New "ImageButton" {
        [Children] = {
            children,

            scope:New "StyleLink" { StyleSheet = Styles }
        }
    }
end