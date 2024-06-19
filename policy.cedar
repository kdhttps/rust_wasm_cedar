permit(
    principal == User::"alice",
    action == Action::"read",
    resource == Resource::"document"
);

permit(
    principal == User::"bob",
    action == Action::"write",
    resource == Resource::"document"
);
