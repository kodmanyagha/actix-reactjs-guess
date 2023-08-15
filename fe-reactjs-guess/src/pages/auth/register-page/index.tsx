import { useEffect } from "react";
import { Button, Col, Form, Row } from "react-bootstrap";
import { useNavigate } from "react-router-dom";
import useApi from "../../../hooks/useApi";
import { registerAction } from "../../../redux/authSlice";
import { useAppDispatch } from "../../../redux/store";

export default function RegisterPage() {
  const navigate = useNavigate();
  const api = useApi();
  const dispatch = useAppDispatch();

  useEffect(() => {
    (async () => {
      //
    })();
  }, []);

  async function onFormSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();

    const formJson = { ...Object.fromEntries(new FormData(e.currentTarget).entries()) };
    const result = await dispatch(registerAction(formJson));
    console.log(">>>  result:", result);
    //navigate("/user");
  }

  return (
    <>
      <Row className="justify-content-center">
        <Col sm="12" md="12" lg="6">
          <Form onSubmit={onFormSubmit}>
            <Form.Group className="mb-3" controlId="registerForm.username">
              <Form.Label>Username</Form.Label>
              <Form.Control
                type="text"
                name="username"
                placeholder="Username"
              />
            </Form.Group>
            <Form.Group className="mb-3" controlId="registerForm.password">
              <Form.Label>Password</Form.Label>
              <Form.Control
                type="password"
                name="password"
                placeholder="Password here..."
              />
            </Form.Group>

            <hr />
            <Form.Group className="mb-3" controlId="registerForm.firstname">
              <Form.Label>First name</Form.Label>
              <Form.Control
                type="text"
                name="firstname"
                placeholder="First name"
              />
            </Form.Group>
            <Form.Group className="mb-3" controlId="registerForm.lastname">
              <Form.Label>Email address</Form.Label>
              <Form.Control
                type="text"
                name="lastname"
                placeholder="Last name"
              />
            </Form.Group>

            <Form.Group className="mb-3" controlId="loginForm.password">
              <Button type="submit" variant="primary" className="w-100 mt-3">
                <i className="fa-solid fa-right-to-bracket" />
                &nbsp; Register
              </Button>
            </Form.Group>
          </Form>
        </Col>
      </Row>
    </>
  );
}
