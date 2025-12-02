# Root URL path should fail
Invoke-RestMethod -Uri http://localhost:4734/

# Get vehicle basic example
Invoke-RestMethod -Uri http://localhost:4734/vehicle -Method Get

# Post new vehicle basic example
Invoke-RestMethod -Uri http://localhost:4734/vehicle -Method Post


# Uses query string parameteres instead of body payload
$QueryParams = @{
Uri = 'http://localhost:4734/vehicle?manufacturer=Toyota&model=Camry&year=2025'
Method = 'Post'
ContentType = 'application/json'
}
Invoke-RestMethod @QueryParams

$Params = @{
Uri = 'http://localhost:4734/vehicle'
Method = 'Post'
Body = @{
    manufacturer = 'Honda'
    model = 'Civic'
    year = 2026
} | ConvertTo-Json
ContentType = 'application/json'
}

Invoke-RestMethod @Params

$QueryCustomerParams = @{
Uri = 'http://localhost:4734/vehicle?manufacturer=Toyota&model=Camry&year=2025&first_name=Sara&last_name=Baamara'
Method = 'Post'
ContentType = 'application/json'
}
Invoke-RestMethod @QueryCustomerParams
