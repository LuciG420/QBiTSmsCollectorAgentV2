function control_flow(event)
    if event.agent_id == "agent123" then
        return "Process"
    else
        return "Ignore"
    end
end
