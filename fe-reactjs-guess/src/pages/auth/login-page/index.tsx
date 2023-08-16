import { unwrapResult } from "@reduxjs/toolkit";
import { Col, Row } from "react-bootstrap";
import Button from "react-bootstrap/Button";
import Form from "react-bootstrap/Form";
import { useNavigate } from "react-router-dom";
import { loginAction } from "../../../features/state/authSlice";
import { useAppDispatch } from "../../../features/state/store";
import { handleAuthResult } from "../../../features/types";

export default function LoginPage() {
  const navigate = useNavigate();
  const dispatch = useAppDispatch();

  async function onFormSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();

    try {
      const formJson = { ...Object.fromEntries(new FormData(e.currentTarget).entries()) };
      const result = unwrapResult(await dispatch(loginAction(formJson)));
      handleAuthResult(result, navigate);
    } catch (e) {
      handleAuthResult(e, navigate);
    }
  }

  return (
    <>
      <Row className="justify-content-center">
        <Col sm="12" md="12" lg="6">
          <Form onSubmit={onFormSubmit}>
            <Form.Group className="mb-3" controlId="loginForm.username">
              <Form.Label>Username</Form.Label>
              <Form.Control
                onChange={(e) => {
                  e.target.value;
                }}
                type="text"
                name="username"
                placeholder="Your username"
              />
            </Form.Group>
            <Form.Group className="mb-3" controlId="loginForm.password">
              <Form.Label>Password</Form.Label>
              <Form.Control
                type="password"
                name="password"
                placeholder="Password here..."
              />
            </Form.Group>

            <Form.Group className="mb-3" controlId="loginForm.password">
              <Button type="submit" variant="primary" className="w-100 mt-3">
                <i className="fa-solid fa-right-to-bracket" />
                &nbsp; Login
              </Button>
            </Form.Group>
          </Form>
        </Col>
      </Row>
    </>
  );
}
