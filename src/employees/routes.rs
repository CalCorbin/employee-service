#[get("/employees")]
/// This functions finds all employees.
///
/// Additionally, we make use of 'spawn blocking' in this function.
/// Spawn blocking is executed via 'web::block', this ensures requests to this route do not block
/// other task executions when we're querying huge records from the database.
async fn find_all() -> Result<HttpResponse, CustomError> {
    let employees = web::block(|| Employees::find_all()).await.unwrap();
    Ok(HttpResponse::Ok().json(employees))
}

#[get("/employees/{id}")]
/// This function finds a single employee by id.
async fn find(id: web::Path) -> Result<HttpResponse, CustomError> {
    let employee = Employees::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[post("/employees")]
/// This function creates a new employee.
async fn create(employee: web::Json) -> Result<HttpResponse, CustomError> {
    let employee = Employees::create(employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[put("/employees/{id}")]
/// This function updates a employee by id.
async fn update(
    id web::Path,
    employee web::Json,
) -> Result<HttpResponse, CustomError> {
    let employee = Employees::update(id.into_inner(), employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[delete("/employees/{id}")]
/// This function deletes a single employee.
async fn delete(id: web::Path) -> Result<HttpResponse, CustomError> {
    let deleted_employee = Employees::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({"deleted": deleted_employee})))
}

/// This function registers our routes.
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
